//! Forked from <https://github.com/tree-sitter/tree-sitter/blob/v0.25.2/highlight/src/lib.rs>
//!
//! The MIT License (MIT)
//!
//! Copyright (c) 2018-2024 Max Brunsfeld
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

#[cfg(all(not(feature = "runtime-c"), not(feature = "runtime-c2rust")))]
compile_error!("Either `runtime-c` or `runtime-c2rust` must be enabled!");
#[cfg(feature = "runtime-c")]
use tree_sitter as ts_runtime;
#[cfg(all(
    feature = "runtime-c2rust",
    not(feature = "runtime-c"), // if both features are enabled, use the c runtime
))]
use tree_sitter_c2rust as ts_runtime;

use regex::Regex;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{iter, mem, ops, slice, str};
use streaming_iterator::StreamingIterator as _;
use thiserror::Error;
use ts_runtime::{
    ffi, Language, Node, ParseOptions, Parser, Point, Query, QueryCapture, QueryCaptures,
    QueryCursor, QueryError, QueryMatch, QueryPredicateArg, Range, TextProvider, Tree,
};

const CANCELLATION_CHECK_INTERVAL: usize = 100;

/// Indicates which highlight should be applied to a region of source code.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Highlight(pub usize);

/// Represents the reason why syntax highlighting failed.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// Highlighting was manually cancelled by flipping the cancellation flag.
    #[error("cancelled")]
    Cancelled,

    /// The provided language uses an incompatible version of tree-sitter.
    #[error("invalid language: incompatible tree-sitter version")]
    InvalidLanguage,
}

/// Represents a single step in rendering a syntax-highlighted document.
#[derive(Copy, Clone, Debug)]
pub enum HighlightEvent {
    Source { start: usize, end: usize },
    HighlightStart(Highlight),
    HighlightEnd,
}

/// Contains the data needed to highlight code written in a particular language.
///
/// This struct is immutable and can be shared between threads.
pub struct HighlightConfiguration {
    pub language: Language,
    pub language_name: String,
    pub query: Query,
    combined_injections_query: Option<Query>,
    locals_pattern_index: usize,
    highlights_pattern_index: usize,
    highlight_indices: Vec<Option<Highlight>>,
    non_local_variable_patterns: Vec<bool>,
    injection_content_capture_index: Option<u32>,
    injection_language_capture_index: Option<u32>,
    local_scope_capture_index: Option<u32>,
    local_def_capture_index: Option<u32>,
    local_def_value_capture_index: Option<u32>,
    local_ref_capture_index: Option<u32>,
}

/// Performs syntax highlighting, recognizing a given list of highlight names.
///
/// For the best performance `Highlighter` values should be reused between
/// syntax highlighting calls. A separate highlighter is needed for each thread that
/// is performing highlighting.
pub struct Highlighter {
    pub parser: Parser,
    cursors: Vec<QueryCursor>,
}

#[derive(Debug)]
struct LocalDef<'a> {
    name: &'a str,
    value_range: ops::Range<usize>,
    highlight: Option<Highlight>,
}

#[derive(Debug)]
struct LocalScope<'a> {
    inherits: bool,
    range: ops::Range<usize>,
    local_defs: Vec<LocalDef<'a>>,
}

struct HighlightIter<'a, F>
where
    F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
{
    source: &'a [u8],
    language_name: &'a str,
    byte_offset: usize,
    highlighter: &'a mut Highlighter,
    injection_callback: F,
    cancellation_flag: Option<&'a AtomicUsize>,
    layers: Vec<HighlightIterLayer<'a>>,
    iter_count: usize,
    next_event: Option<HighlightEvent>,
    last_highlight_range: Option<(usize, usize, usize)>,
}

struct HighlightIterLayer<'a> {
    _tree: Option<Tree>, // needed to keep tree in memory
    cursor: QueryCursor,
    captures: iter::Peekable<_QueryCaptures<'a, 'a, &'a [u8], &'a [u8]>>,
    config: &'a HighlightConfiguration,
    highlight_end_stack: Vec<usize>,
    scope_stack: Vec<LocalScope<'a>>,
    ranges: Vec<Range>,
    depth: usize,
}

// Additional option to only include unnamed children. Borrowed from helix-editor.
// (syntastica addition)
#[derive(Clone, Copy)]
enum IncludedChildren {
    None,
    All,
    Unnamed,
}

pub struct _QueryCaptures<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> {
    ptr: *mut ffi::TSQueryCursor,
    query: &'query Query,
    text_provider: T,
    buffer1: Vec<u8>,
    buffer2: Vec<u8>,
    _current_match: Option<(QueryMatch<'query, 'tree>, usize)>,
    _options: Option<*mut ffi::TSQueryCursorOptions>,
    _phantom: PhantomData<(&'tree (), I)>,
}

struct _QueryMatch<'cursor, 'tree> {
    pub _pattern_index: usize,
    pub _captures: &'cursor [QueryCapture<'tree>],
    _id: u32,
    _cursor: *mut ffi::TSQueryCursor,
}

impl<'tree> _QueryMatch<'_, 'tree> {
    fn new(m: &ffi::TSQueryMatch, cursor: *mut ffi::TSQueryCursor) -> Self {
        _QueryMatch {
            _cursor: cursor,
            _id: m.id,
            _pattern_index: m.pattern_index as usize,
            _captures: (m.capture_count > 0)
                .then(|| unsafe {
                    slice::from_raw_parts(
                        m.captures.cast::<QueryCapture<'tree>>(),
                        m.capture_count as usize,
                    )
                })
                .unwrap_or_default(),
        }
    }
}

impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> Iterator
    for _QueryCaptures<'query, 'tree, T, I>
{
    type Item = (QueryMatch<'query, 'tree>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                let mut capture_index = 0u32;
                let mut m = MaybeUninit::<ffi::TSQueryMatch>::uninit();
                if ffi::ts_query_cursor_next_capture(
                    self.ptr,
                    m.as_mut_ptr(),
                    core::ptr::addr_of_mut!(capture_index),
                ) {
                    let result = std::mem::transmute::<_QueryMatch, QueryMatch>(_QueryMatch::new(
                        &m.assume_init(),
                        self.ptr,
                    ));
                    if result.satisfies_text_predicates(
                        self.query,
                        &mut self.buffer1,
                        &mut self.buffer2,
                        &mut self.text_provider,
                    ) {
                        return Some((result, capture_index as usize));
                    }
                    result.remove();
                } else {
                    return None;
                }
            }
        }
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            cursors: Vec::new(),
        }
    }

    pub fn parser(&mut self) -> &mut Parser {
        &mut self.parser
    }

    /// Iterate over the highlighted regions for a given slice of source code.
    pub fn highlight<'a>(
        &'a mut self,
        config: &'a HighlightConfiguration,
        source: &'a [u8],
        cancellation_flag: Option<&'a AtomicUsize>,
        mut injection_callback: impl FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
    ) -> Result<impl Iterator<Item = Result<HighlightEvent, Error>> + 'a, Error> {
        let layers = HighlightIterLayer::new(
            source,
            None,
            self,
            cancellation_flag,
            &mut injection_callback,
            config,
            0,
            vec![Range {
                start_byte: 0,
                end_byte: usize::MAX,
                start_point: Point::new(0, 0),
                end_point: Point::new(usize::MAX, usize::MAX),
            }],
        )?;
        assert_ne!(layers.len(), 0);
        let mut result = HighlightIter {
            source,
            language_name: &config.language_name,
            byte_offset: 0,
            injection_callback,
            cancellation_flag,
            highlighter: self,
            iter_count: 0,
            layers,
            next_event: None,
            last_highlight_range: None,
        };
        result.sort_layers();
        Ok(result)
    }

    /// Iterate over the highlighted regions for a given node.
    /// Does not parse anything (and therefore does not support injections).
    // (syntastica addition)
    pub fn highlight_existing_tree<'a>(
        &'a mut self,
        config: &'a HighlightConfiguration,
        source: &'a [u8],
        cancellation_flag: Option<&'a AtomicUsize>,
        node: &'a Node,
    ) -> Result<impl Iterator<Item = Result<HighlightEvent, Error>> + 'a, Error> {
        let layers = vec![HighlightIterLayer::new_from_tree(source, node, config)];
        let mut result = HighlightIter {
            source,
            language_name: &config.language_name,
            byte_offset: 0,
            injection_callback: |_| None,
            cancellation_flag,
            highlighter: self,
            iter_count: 0,
            layers,
            next_event: None,
            last_highlight_range: None,
        };
        result.sort_layers();
        Ok(result)
    }
}

impl HighlightConfiguration {
    /// Creates a `HighlightConfiguration` for a given `Language` and set of highlighting
    /// queries.
    ///
    /// # Parameters
    ///
    /// * `language`  - The Tree-sitter `Language` that should be used for parsing.
    /// * `highlights_query` - A string containing tree patterns for syntax highlighting. This
    ///   should be non-empty, otherwise no syntax highlights will be added.
    /// * `injections_query` -  A string containing tree patterns for injecting other languages into
    ///   the document. This can be empty if no injections are desired.
    /// * `locals_query` - A string containing tree patterns for tracking local variable definitions
    ///   and references. This can be empty if local variable tracking is not needed.
    ///
    /// Returns a `HighlightConfiguration` that can then be used with the `highlight` method.
    pub fn new(
        language: Language,
        name: impl Into<String>,
        highlights_query: &str,
        injection_query: &str,
        locals_query: &str,
    ) -> Result<Self, QueryError> {
        // Concatenate the query strings, keeping track of the start offset of each section.
        let mut query_source = String::new();
        query_source.push_str(injection_query);
        let locals_query_offset = query_source.len();
        query_source.push_str(locals_query);
        let highlights_query_offset = query_source.len();
        query_source.push_str(highlights_query);

        // Construct a single query by concatenating the three query strings, but record the
        // range of pattern indices that belong to each individual string.
        let mut query = Query::new(&language, &query_source)?;
        let mut locals_pattern_index = 0;
        let mut highlights_pattern_index = 0;
        for i in 0..(query.pattern_count()) {
            let pattern_offset = query.start_byte_for_pattern(i);
            if pattern_offset < highlights_query_offset {
                if pattern_offset < highlights_query_offset {
                    highlights_pattern_index += 1;
                }
                if pattern_offset < locals_query_offset {
                    locals_pattern_index += 1;
                }
            }
        }

        // Construct a separate query just for dealing with the 'combined injections'.
        // Disable the combined injection patterns in the main query.
        let mut combined_injections_query = Query::new(&language, injection_query)?;
        let mut has_combined_queries = false;
        for pattern_index in 0..locals_pattern_index {
            let settings = query.property_settings(pattern_index);
            if settings.iter().any(|s| &*s.key == "injection.combined") {
                has_combined_queries = true;
                query.disable_pattern(pattern_index);
            } else {
                combined_injections_query.disable_pattern(pattern_index);
            }
        }
        let combined_injections_query = if has_combined_queries {
            Some(combined_injections_query)
        } else {
            None
        };

        // Find all of the highlighting patterns that are disabled for nodes that
        // have been identified as local variables.
        let non_local_variable_patterns = (0..query.pattern_count())
            .map(|i| {
                query
                    .property_predicates(i)
                    .iter()
                    .any(|(prop, positive)| !*positive && prop.key.as_ref() == "local")
            })
            .collect();

        // Store the numeric ids for all of the special captures.
        let mut injection_content_capture_index = None;
        let mut injection_language_capture_index = None;
        let mut local_def_capture_index = None;
        let mut local_def_value_capture_index = None;
        let mut local_ref_capture_index = None;
        let mut local_scope_capture_index = None;
        for (i, name) in query.capture_names().iter().enumerate() {
            let i = Some(i as u32);
            match *name {
                "injection.content" => injection_content_capture_index = i,
                "injection.language" => injection_language_capture_index = i,
                "local.definition" => local_def_capture_index = i,
                "local.definition-value" => local_def_value_capture_index = i,
                "local.reference" => local_ref_capture_index = i,
                "local.scope" => local_scope_capture_index = i,
                _ => {}
            }
        }

        let highlight_indices = vec![None; query.capture_names().len()];
        Ok(Self {
            language,
            language_name: name.into(),
            query,
            combined_injections_query,
            locals_pattern_index,
            highlights_pattern_index,
            highlight_indices,
            non_local_variable_patterns,
            injection_content_capture_index,
            injection_language_capture_index,
            local_def_capture_index,
            local_def_value_capture_index,
            local_ref_capture_index,
            local_scope_capture_index,
        })
    }

    /// Get a slice containing all of the highlight names used in the configuration.
    #[must_use]
    pub const fn names(&self) -> &[&str] {
        self.query.capture_names()
    }

    /// Set the list of recognized highlight names.
    ///
    /// Tree-sitter syntax-highlighting queries specify highlights in the form of dot-separated
    /// highlight names like `punctuation.bracket` and `function.method.builtin`. Consumers of
    /// these queries can choose to recognize highlights with different levels of specificity.
    /// For example, the string `function.builtin` will match against `function.method.builtin`
    /// and `function.builtin.constructor`, but will not match `function.method`.
    ///
    /// When highlighting, results are returned as `Highlight` values, which contain the index
    /// of the matched highlight this list of highlight names.
    pub fn configure(&mut self, recognized_names: &[impl AsRef<str>]) {
        let mut capture_parts = Vec::new();
        self.highlight_indices.clear();
        self.highlight_indices
            .extend(self.query.capture_names().iter().map(move |capture_name| {
                capture_parts.clear();
                capture_parts.extend(capture_name.split('.'));

                let mut best_index = None;
                let mut best_match_len = 0;
                for (i, recognized_name) in recognized_names.iter().enumerate() {
                    let mut len = 0;
                    let mut matches = true;
                    for part in recognized_name.as_ref().split('.') {
                        len += 1;
                        if !capture_parts.contains(&part) {
                            matches = false;
                            break;
                        }
                    }
                    if matches && len > best_match_len {
                        best_index = Some(i);
                        best_match_len = len;
                    }
                }
                best_index.map(Highlight)
            }));
    }
}

impl<'a> HighlightIterLayer<'a> {
    /// Create a new 'layer' of highlighting for this document.
    ///
    /// In the event that the new layer contains "combined injections" (injections where multiple
    /// disjoint ranges are parsed as one syntax tree), these will be eagerly processed and
    /// added to the returned vector.
    #[allow(clippy::too_many_arguments)]
    fn new<F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a>(
        source: &'a [u8],
        parent_name: Option<&str>,
        highlighter: &mut Highlighter,
        cancellation_flag: Option<&'a AtomicUsize>,
        injection_callback: &mut F,
        mut config: &'a HighlightConfiguration,
        mut depth: usize,
        mut ranges: Vec<Range>,
    ) -> Result<Vec<Self>, Error> {
        let mut result = Vec::with_capacity(1);
        let mut queue = Vec::new();
        loop {
            if highlighter.parser.set_included_ranges(&ranges).is_ok() {
                highlighter
                    .parser
                    .set_language(&config.language)
                    .map_err(|_| Error::InvalidLanguage)?;

                let tree = highlighter
                    .parser
                    .parse_with_options(
                        &mut |i, _| if i < source.len() { &source[i..] } else { &[] },
                        None,
                        Some(ParseOptions::new().progress_callback(&mut |_| {
                            if let Some(cancellation_flag) = cancellation_flag {
                                cancellation_flag.load(Ordering::SeqCst) != 0
                            } else {
                                false
                            }
                        })),
                    )
                    .ok_or(Error::Cancelled)?;
                let mut cursor = highlighter.cursors.pop().unwrap_or_default();

                // Process combined injections.
                if let Some(combined_injections_query) = &config.combined_injections_query {
                    let mut injections_by_pattern_index =
                        vec![
                            (None, Vec::new(), IncludedChildren::None);
                            combined_injections_query.pattern_count()
                        ];
                    let mut matches =
                        cursor.matches(combined_injections_query, tree.root_node(), source);
                    while let Some(mat) = matches.next() {
                        let entry = &mut injections_by_pattern_index[mat.pattern_index];
                        let (language_name, content_node, included_children) = injection_for_match(
                            config,
                            parent_name,
                            combined_injections_query,
                            mat,
                            source,
                        );
                        if language_name.is_some() {
                            entry.0 = language_name;
                        }
                        if let Some(content_node) = content_node {
                            entry.1.push(content_node);
                        }
                        entry.2 = included_children;
                    }
                    for (lang_name, content_nodes, included_children) in injections_by_pattern_index
                    {
                        if let (Some(lang_name), false) = (lang_name, content_nodes.is_empty()) {
                            if let Some(next_config) = (injection_callback)(&lang_name) {
                                let ranges = Self::intersect_ranges(
                                    &ranges,
                                    &content_nodes,
                                    included_children,
                                );
                                if !ranges.is_empty() {
                                    queue.push((next_config, depth + 1, ranges));
                                }
                            }
                        }
                    }
                }

                // The `captures` iterator borrows the `Tree` and the `QueryCursor`, which
                // prevents them from being moved. But both of these values are really just
                // pointers, so it's actually ok to move them.
                let tree_ref = unsafe { mem::transmute::<&Tree, &'static Tree>(&tree) };
                let cursor_ref = unsafe {
                    mem::transmute::<&mut QueryCursor, &'static mut QueryCursor>(&mut cursor)
                };
                let captures = unsafe {
                    mem::transmute::<QueryCaptures<_, _>, _QueryCaptures<_, _>>(
                        cursor_ref.captures(&config.query, tree_ref.root_node(), source),
                    )
                }
                .peekable();

                result.push(HighlightIterLayer {
                    highlight_end_stack: Vec::new(),
                    scope_stack: vec![LocalScope {
                        inherits: false,
                        range: 0..usize::MAX,
                        local_defs: Vec::new(),
                    }],
                    cursor,
                    depth,
                    _tree: Some(tree),
                    captures,
                    config,
                    ranges,
                });
            }

            if queue.is_empty() {
                break;
            }

            let (next_config, next_depth, next_ranges) = queue.remove(0);
            config = next_config;
            depth = next_depth;
            ranges = next_ranges;
        }

        Ok(result)
    }

    // (syntastica addition)
    fn new_from_tree(source: &'a [u8], node: &'a Node, config: &'a HighlightConfiguration) -> Self {
        let mut cursor = QueryCursor::new();

        // `QueryCursor` is really just a pointer, so it's ok to move.
        let cursor_ref =
            unsafe { mem::transmute::<&mut QueryCursor, &'static mut QueryCursor>(&mut cursor) };
        let captures = unsafe {
            mem::transmute::<QueryCaptures<_, _>, _QueryCaptures<_, _>>(cursor_ref.captures(
                &config.query,
                *node,
                source,
            ))
        }
        .peekable();

        HighlightIterLayer {
            highlight_end_stack: Vec::new(),
            scope_stack: vec![LocalScope {
                inherits: false,
                range: 0..usize::MAX,
                local_defs: Vec::new(),
            }],
            cursor,
            depth: 0,
            _tree: None,
            captures,
            config,
            ranges: vec![Range {
                start_byte: 0,
                end_byte: usize::MAX,
                start_point: Point::new(0, 0),
                end_point: Point::new(usize::MAX, usize::MAX),
            }],
        }
    }

    // Compute the ranges that should be included when parsing an injection.
    // This takes into account three things:
    // * `parent_ranges` - The ranges must all fall within the *current* layer's ranges.
    // * `nodes` - Every injection takes place within a set of nodes. The injection ranges are the
    //   ranges of those nodes.
    // * `included_children` - For some injections, the content nodes' children should be excluded
    //   from the nested document, so that only the content nodes' *own* content is reparsed. For
    //   other injections, the content nodes' entire ranges should be reparsed, including the ranges
    //   of their children.
    fn intersect_ranges(
        parent_ranges: &[Range],
        nodes: &[Node],
        included_children: IncludedChildren,
    ) -> Vec<Range> {
        let mut cursor = nodes[0].walk();
        let mut result = Vec::new();
        let mut parent_range_iter = parent_ranges.iter();
        let mut parent_range = parent_range_iter
            .next()
            .expect("Layers should only be constructed with non-empty ranges vectors");
        for node in nodes {
            let mut preceding_range = Range {
                start_byte: 0,
                start_point: Point::new(0, 0),
                end_byte: node.start_byte(),
                end_point: node.start_position(),
            };
            let following_range = Range {
                start_byte: node.end_byte(),
                start_point: node.end_position(),
                end_byte: usize::MAX,
                end_point: Point::new(usize::MAX, usize::MAX),
            };

            for excluded_range in node
                .children(&mut cursor)
                .filter_map(|child| match included_children {
                    IncludedChildren::None => Some(child.range()),
                    IncludedChildren::All => None,
                    IncludedChildren::Unnamed => {
                        if child.is_named() {
                            Some(child.range())
                        } else {
                            None
                        }
                    }
                })
                .chain(std::iter::once(following_range))
            {
                let mut range = Range {
                    start_byte: preceding_range.end_byte,
                    start_point: preceding_range.end_point,
                    end_byte: excluded_range.start_byte,
                    end_point: excluded_range.start_point,
                };
                preceding_range = excluded_range;

                if range.end_byte < parent_range.start_byte {
                    continue;
                }

                while parent_range.start_byte <= range.end_byte {
                    if parent_range.end_byte > range.start_byte {
                        if range.start_byte < parent_range.start_byte {
                            range.start_byte = parent_range.start_byte;
                            range.start_point = parent_range.start_point;
                        }

                        if parent_range.end_byte < range.end_byte {
                            if range.start_byte < parent_range.end_byte {
                                result.push(Range {
                                    start_byte: range.start_byte,
                                    start_point: range.start_point,
                                    end_byte: parent_range.end_byte,
                                    end_point: parent_range.end_point,
                                });
                            }
                            range.start_byte = parent_range.end_byte;
                            range.start_point = parent_range.end_point;
                        } else {
                            if range.start_byte < range.end_byte {
                                result.push(range);
                            }
                            break;
                        }
                    }

                    if let Some(next_range) = parent_range_iter.next() {
                        parent_range = next_range;
                    } else {
                        return result;
                    }
                }
            }
        }
        result
    }

    // First, sort scope boundaries by their byte offset in the document. At a
    // given position, emit scope endings before scope beginnings. Finally, emit
    // scope boundaries from deeper layers first.
    fn sort_key(&mut self) -> Option<(usize, bool, isize)> {
        let depth = -(self.depth as isize);
        let next_start = self
            .captures
            .peek()
            .map(|(m, i)| m.captures[*i].node.start_byte());
        let next_end = self.highlight_end_stack.last().copied();
        match (next_start, next_end) {
            (Some(start), Some(end)) => {
                if start < end {
                    Some((start, true, depth))
                } else {
                    Some((end, false, depth))
                }
            }
            (Some(i), None) => Some((i, true, depth)),
            (None, Some(j)) => Some((j, false, depth)),
            _ => None,
        }
    }
}

impl<'a, F> HighlightIter<'a, F>
where
    F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
{
    fn emit_event(
        &mut self,
        offset: usize,
        event: Option<HighlightEvent>,
    ) -> Option<Result<HighlightEvent, Error>> {
        let result;
        if self.byte_offset < offset {
            result = Some(Ok(HighlightEvent::Source {
                start: self.byte_offset,
                end: offset,
            }));
            self.byte_offset = offset;
            self.next_event = event;
        } else {
            result = event.map(Ok);
        }
        self.sort_layers();
        result
    }

    fn sort_layers(&mut self) {
        while !self.layers.is_empty() {
            if let Some(sort_key) = self.layers[0].sort_key() {
                let mut i = 0;
                while i + 1 < self.layers.len() {
                    if let Some(next_offset) = self.layers[i + 1].sort_key() {
                        if next_offset < sort_key {
                            i += 1;
                            continue;
                        }
                    }
                    break;
                }
                if i > 0 {
                    self.layers[0..=i].rotate_left(1);
                }
                break;
            }
            let layer = self.layers.remove(0);
            self.highlighter.cursors.push(layer.cursor);
        }
    }

    fn insert_layer(&mut self, mut layer: HighlightIterLayer<'a>) {
        if let Some(sort_key) = layer.sort_key() {
            let mut i = 1;
            while i < self.layers.len() {
                if let Some(sort_key_i) = self.layers[i].sort_key() {
                    if sort_key_i > sort_key {
                        self.layers.insert(i, layer);
                        return;
                    }
                    i += 1;
                } else {
                    self.layers.remove(i);
                }
            }
            self.layers.push(layer);
        }
    }
}

impl<'a, F> Iterator for HighlightIter<'a, F>
where
    F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
{
    type Item = Result<HighlightEvent, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        'main: loop {
            // If we've already determined the next highlight boundary, just return it.
            if let Some(e) = self.next_event.take() {
                return Some(Ok(e));
            }

            // Periodically check for cancellation, returning `Cancelled` error if the
            // cancellation flag was flipped.
            if let Some(cancellation_flag) = self.cancellation_flag {
                self.iter_count += 1;
                if self.iter_count >= CANCELLATION_CHECK_INTERVAL {
                    self.iter_count = 0;
                    if cancellation_flag.load(Ordering::Relaxed) != 0 {
                        return Some(Err(Error::Cancelled));
                    }
                }
            }

            // If none of the layers have any more highlight boundaries, terminate.
            if self.layers.is_empty() {
                return if self.byte_offset < self.source.len() {
                    let result = Some(Ok(HighlightEvent::Source {
                        start: self.byte_offset,
                        end: self.source.len(),
                    }));
                    self.byte_offset = self.source.len();
                    result
                } else {
                    None
                };
            }

            // Get the next capture from whichever layer has the earliest highlight boundary.
            let range;
            let layer = &mut self.layers[0];
            if let Some((next_match, capture_index)) = layer.captures.peek() {
                let next_capture = next_match.captures[*capture_index];
                range = next_capture.node.byte_range();

                // If any previous highlight ends before this node starts, then before
                // processing this capture, emit the source code up until the end of the
                // previous highlight, and an end event for that highlight.
                if let Some(end_byte) = layer.highlight_end_stack.last().copied() {
                    if end_byte <= range.start {
                        layer.highlight_end_stack.pop();
                        return self.emit_event(end_byte, Some(HighlightEvent::HighlightEnd));
                    }
                }
            }
            // If there are no more captures, then emit any remaining highlight end events.
            // And if there are none of those, then just advance to the end of the document.
            else if let Some(end_byte) = layer.highlight_end_stack.last().copied() {
                layer.highlight_end_stack.pop();
                return self.emit_event(end_byte, Some(HighlightEvent::HighlightEnd));
            } else {
                return self.emit_event(self.source.len(), None);
            }

            let (mut match_, capture_index) = layer.captures.next().unwrap();
            let mut capture = match_.captures[capture_index];

            // If this capture represents an injection, then process the injection.
            if match_.pattern_index < layer.config.locals_pattern_index {
                let (language_name, content_node, included_children) = injection_for_match(
                    layer.config,
                    Some(self.language_name),
                    &layer.config.query,
                    &match_,
                    self.source,
                );

                // If a language is found with the given name, then add a new language layer
                // to the highlighted document.
                if let (Some(language_name), Some(content_node)) = (language_name, content_node) {
                    // Explicitly remove this match so that none of its other captures will remain
                    // in the stream of captures.
                    match_.remove();

                    if let Some(config) = (self.injection_callback)(&language_name) {
                        let ranges = HighlightIterLayer::intersect_ranges(
                            &self.layers[0].ranges,
                            &[content_node],
                            included_children,
                        );
                        if !ranges.is_empty() {
                            match HighlightIterLayer::new(
                                self.source,
                                Some(self.language_name),
                                self.highlighter,
                                self.cancellation_flag,
                                &mut self.injection_callback,
                                config,
                                self.layers[0].depth + 1,
                                ranges,
                            ) {
                                Ok(layers) => {
                                    for layer in layers {
                                        self.insert_layer(layer);
                                    }
                                }
                                Err(e) => return Some(Err(e)),
                            }
                        }
                    }
                }

                self.sort_layers();
                continue 'main;
            }

            // Remove from the local scope stack any local scopes that have already ended.
            while range.start > layer.scope_stack.last().unwrap().range.end {
                layer.scope_stack.pop();
            }

            // If this capture is for tracking local variables, then process the
            // local variable info.
            let mut reference_highlight = None;
            let mut definition_highlight = None;
            while match_.pattern_index < layer.config.highlights_pattern_index {
                // If the node represents a local scope, push a new local scope onto
                // the scope stack.
                if Some(capture.index) == layer.config.local_scope_capture_index {
                    definition_highlight = None;
                    let mut scope = LocalScope {
                        inherits: true,
                        range: range.clone(),
                        local_defs: Vec::new(),
                    };
                    for prop in layer.config.query.property_settings(match_.pattern_index) {
                        if prop.key.as_ref() == "local.scope-inherits" {
                            scope.inherits =
                                prop.value.as_ref().is_none_or(|r| r.as_ref() == "true");
                        }
                    }
                    layer.scope_stack.push(scope);
                }
                // If the node represents a definition, add a new definition to the
                // local scope at the top of the scope stack.
                else if Some(capture.index) == layer.config.local_def_capture_index {
                    reference_highlight = None;
                    definition_highlight = None;
                    let scope = layer.scope_stack.last_mut().unwrap();

                    let mut value_range = 0..0;
                    for capture in match_.captures {
                        if Some(capture.index) == layer.config.local_def_value_capture_index {
                            value_range = capture.node.byte_range();
                        }
                    }

                    if let Ok(name) = str::from_utf8(&self.source[range.clone()]) {
                        scope.local_defs.push(LocalDef {
                            name,
                            value_range,
                            highlight: None,
                        });
                        definition_highlight =
                            scope.local_defs.last_mut().map(|s| &mut s.highlight);
                    }
                }
                // If the node represents a reference, then try to find the corresponding
                // definition in the scope stack.
                else if Some(capture.index) == layer.config.local_ref_capture_index
                    && definition_highlight.is_none()
                {
                    definition_highlight = None;
                    if let Ok(name) = str::from_utf8(&self.source[range.clone()]) {
                        for scope in layer.scope_stack.iter().rev() {
                            if let Some(highlight) = scope.local_defs.iter().rev().find_map(|def| {
                                if def.name == name && range.start >= def.value_range.end {
                                    Some(def.highlight)
                                } else {
                                    None
                                }
                            }) {
                                reference_highlight = highlight;
                                break;
                            }
                            if !scope.inherits {
                                break;
                            }
                        }
                    }
                }

                // Continue processing any additional matches for the same node.
                if let Some((next_match, next_capture_index)) = layer.captures.peek() {
                    let next_capture = next_match.captures[*next_capture_index];
                    if next_capture.node == capture.node {
                        capture = next_capture;
                        match_ = layer.captures.next().unwrap().0;
                        continue;
                    }
                }

                self.sort_layers();
                continue 'main;
            }

            // Otherwise, this capture must represent a highlight.
            // If this exact range has already been highlighted by an earlier pattern, or by
            // a different layer, then skip over this one.
            if let Some((last_start, last_end, last_depth)) = self.last_highlight_range {
                if range.start == last_start && range.end == last_end && layer.depth < last_depth {
                    self.sort_layers();
                    continue 'main;
                }
            }

            // Once a highlighting pattern is found for the current node, keep iterating over
            // any later highlighting patterns that also match this node and set the match to it.
            // Captures for a given node are ordered by pattern index, so these subsequent
            // captures are guaranteed to be for highlighting, not injections or
            // local variables.
            while let Some((next_match, next_capture_index)) = layer.captures.peek() {
                let next_capture = next_match.captures[*next_capture_index];
                if next_capture.node == capture.node {
                    let (following_match, capture_index) = layer.captures.next().unwrap();
                    // If the current node was found to be a local variable, then ignore
                    // the following match if it's a highlighting pattern that is disabled
                    // for local variables.
                    if (definition_highlight.is_some() || reference_highlight.is_some())
                        && layer.config.non_local_variable_patterns[following_match.pattern_index]
                    {
                        continue;
                    }

                    // Skip captures starting with `_`.
                    // (syntastica addition)
                    if layer.config.names()[following_match.captures[capture_index].index as usize]
                        .starts_with('_')
                    {
                        continue;
                    }

                    match_.remove();
                    capture = next_capture;
                    match_ = following_match;
                } else {
                    break;
                }
            }

            let current_highlight = layer.config.highlight_indices[capture.index as usize];

            // If this node represents a local definition, then store the current
            // highlight value on the local scope entry representing this node.
            if let Some(definition_highlight) = definition_highlight {
                *definition_highlight = current_highlight;
            }

            // Emit a scope start event and push the node's end position to the stack.
            if let Some(highlight) = reference_highlight.or(current_highlight) {
                self.last_highlight_range = Some((range.start, range.end, layer.depth));
                layer.highlight_end_stack.push(range.end);
                return self
                    .emit_event(range.start, Some(HighlightEvent::HighlightStart(highlight)));
            }

            self.sort_layers();
        }
    }
}

fn injection_for_match<'a>(
    config: &'a HighlightConfiguration,
    parent_name: Option<&'a str>,
    query: &'a Query,
    query_match: &QueryMatch<'a, 'a>,
    source: &'a [u8],
) -> (Option<Cow<'a, str>>, Option<Node<'a>>, IncludedChildren) {
    let content_capture_index = config.injection_content_capture_index;
    let language_capture_index = config.injection_language_capture_index;

    let mut language_name = None;
    let mut content_node = None;

    for capture in query_match.captures {
        let index = Some(capture.index);
        if index == language_capture_index {
            language_name = capture.node.utf8_text(source).ok();
        } else if index == content_capture_index {
            content_node = Some(capture.node);
        }
    }

    let mut included_children = IncludedChildren::None;
    for prop in query.property_settings(query_match.pattern_index) {
        match prop.key.as_ref() {
            // In addition to specifying the language name via the text of a
            // captured node, it can also be hard-coded via a `#set!` predicate
            // that sets the injection.language key.
            "injection.language" => {
                if language_name.is_none() {
                    language_name = prop.value.as_ref().map(std::convert::AsRef::as_ref);
                }
            }

            // Setting the `injection.self` key can be used to specify that the
            // language name should be the same as the language of the current
            // layer.
            "injection.self" => {
                if language_name.is_none() {
                    language_name = Some(config.language_name.as_str());
                }
            }

            // Setting the `injection.parent` key can be used to specify that
            // the language name should be the same as the language of the
            // parent layer
            "injection.parent" => {
                if language_name.is_none() {
                    language_name = parent_name;
                }
            }

            // By default, injections do not include the *children* of an
            // `injection.content` node - only the ranges that belong to the
            // node itself. This can be changed using a `#set!` predicate that
            // sets the `injection.include-children` key.
            "injection.include-children" => included_children = IncludedChildren::All,

            // Some queries might only exclude named children but include unnamed
            // children in their `injection.content` node. This can be enabled using
            // a `#set!` predicate that sets the `injection.include-unnamed-children` key.
            // (syntastica addition)
            "injection.include-unnamed-children" => included_children = IncludedChildren::Unnamed,
            _ => {}
        }
    }

    // (syntastica addition)
    let mut language_name = language_name.map(Cow::Borrowed);
    if let Some(language) = &mut language_name {
        for predicate in query.general_predicates(query_match.pattern_index) {
            if predicate.operator == "replace!".into() {
                let [QueryPredicateArg::Capture(capture), QueryPredicateArg::String(pattern), QueryPredicateArg::String(replacement)] =
                    &*predicate.args
                else {
                    // TODO: maybe don't ignore errors
                    continue;
                };
                if Some(*capture) != language_capture_index {
                    continue;
                }
                let Ok(re) = Regex::new(pattern) else {
                    continue;
                };
                *language = re.replace_all(language, &**replacement).into_owned().into();
            }
        }
    }

    (language_name, content_node, included_children)
}
