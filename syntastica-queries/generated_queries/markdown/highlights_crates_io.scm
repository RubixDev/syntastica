[
  (backslash_escape)
] @string.escape

[
  (block_continuation)
  (block_quote_marker)
] @punctuation.special

(block_quote) @text.quote

(task_list_marker_checked) @text.todo.checked

(task_list_marker_unchecked) @text.todo.unchecked

[
  (list_marker_plus)
  (list_marker_minus)
  (list_marker_star)
  (list_marker_dot)
  (list_marker_parenthesis)
  (thematic_break)
] @punctuation.special

[
  (link_label)
] @text.reference

[
  (link_destination)
] @text.uri

[
  (fenced_code_block_delimiter)
] @punctuation.delimiter

(pipe_table_delimiter_cell) @punctuation.special

(pipe_table_delimiter_row
  "|" @punctuation.special
)

(pipe_table_row
  "|" @punctuation.special
)

(pipe_table_header
  "|" @punctuation.special
)

(pipe_table_header
  (pipe_table_cell) @text.title
)

(info_string) @label

(
  (fenced_code_block) @text.literal.block
  (#set! "priority" 90)
)

(indented_code_block) @text.literal.block

(link_title) @text.literal

(atx_heading
  (atx_h6_marker) @text.title.6.marker
  (inline) @text.title.6
)

(atx_heading
  (atx_h5_marker) @text.title.5.marker
  (inline) @text.title.5
)

(atx_heading
  (atx_h4_marker) @text.title.4.marker
  (inline) @text.title.4
)

(atx_heading
  (atx_h3_marker) @text.title.3.marker
  (inline) @text.title.3
)

(atx_heading
  (atx_h2_marker) @text.title.2.marker
  (inline) @text.title.2
)

(atx_heading
  (atx_h1_marker) @text.title.1.marker
  (inline) @text.title.1
)

(setext_heading
  (paragraph) @text.title.2
  (setext_h2_underline) @text.title.2.marker
)

(setext_heading
  (paragraph) @text.title.1
  (setext_h1_underline) @text.title.1.marker
)
