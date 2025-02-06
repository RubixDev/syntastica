;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/latex/highlights.scm
;; Licensed under the Apache License 2.0
; General syntax
(command_name) @function

(caption
  command:
  _ @function
)

; \text, \intertext, \shortintertext, ...
(text_mode
  command:
  _ @function
  content: (curly_group
    (_) @none
  )
)

; Variables, parameters
(placeholder) @variable

(key_value_pair
  key: (_) @variable.parameter
  value: (_)
)

(curly_group_spec
  (text) @variable.parameter
)

(brack_group_argc) @variable.parameter

[
  (operator)
  "="
  "_"
  "^"
] @operator

"\\item" @punctuation.special

(delimiter) @punctuation.delimiter

(math_delimiter
  left_command:
  _ @punctuation.delimiter
  left_delimiter:
  _ @punctuation.delimiter
  right_command:
  _ @punctuation.delimiter
  right_delimiter:
  _ @punctuation.delimiter
)

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; "(" ")" has no syntactical meaning in LaTeX
; General environments
(begin
  command:
  _ @module
  name: (curly_group_text
    (text) @label
  )
)

(end
  command:
  _ @module
  name: (curly_group_text
    (text) @label
  )
)

; Definitions and references
(new_command_definition
  command:
  _ @function.macro
)

(old_command_definition
  command:
  _ @function.macro
)

(let_command_definition
  command:
  _ @function.macro
)

(environment_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @label
  )
)

(theorem_definition
  command:
  _ @function.macro
  name: (curly_group_text_list
    (_) @label
  )
)

(paired_delimiter_definition
  command:
  _ @function.macro
  declaration: (curly_group_command_name
    (_) @function
  )
)

(label_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )
)

(label_reference_range
  command:
  _ @function.macro
  from: (curly_group_text
    (_) @markup.link
  )
  to: (curly_group_text
    (_) @markup.link
  )
)

(label_reference
  command:
  _ @function.macro
  names: (curly_group_text_list
    (_) @markup.link
  )
)

(label_number
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )
  number: (_) @markup.link
)

(citation
  command:
  _ @function.macro
  keys: (curly_group_text_list) @markup.link
)

(
  (hyperlink
    command:
    _ @function
    uri: (curly_group_uri
      (_) @markup.link.url
    )
  ) @_hyperlink
  (#set! @_hyperlink url @markup.link.url)
)

(glossary_entry_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )
)

(glossary_entry_reference
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )
)

(acronym_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )
)

(acronym_reference
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )
)

(color_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )
)

(color_reference
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @markup.link
  )?
)

; Sectioning
(title_declaration
  command:
  _ @module
  options: (brack_group
    (_) @markup.heading.1
  )?
  text: (curly_group
    (_) @markup.heading.1
  )
)

(author_declaration
  command:
  _ @module
  authors: (curly_group_author_list
    (author)+ @markup.heading.1
  )
)

(chapter
  command:
  _ @module
  toc: (brack_group
    (_) @markup.heading.2
  )?
  text: (curly_group
    (_) @markup.heading.2
  )
)

(part
  command:
  _ @module
  toc: (brack_group
    (_) @markup.heading.2
  )?
  text: (curly_group
    (_) @markup.heading.2
  )
)

(section
  command:
  _ @module
  toc: (brack_group
    (_) @markup.heading.3
  )?
  text: (curly_group
    (_) @markup.heading.3
  )
)

(subsection
  command:
  _ @module
  toc: (brack_group
    (_) @markup.heading.4
  )?
  text: (curly_group
    (_) @markup.heading.4
  )
)

(subsubsection
  command:
  _ @module
  toc: (brack_group
    (_) @markup.heading.5
  )?
  text: (curly_group
    (_) @markup.heading.5
  )
)

(paragraph
  command:
  _ @module
  toc: (brack_group
    (_) @markup.heading.6
  )?
  text: (curly_group
    (_) @markup.heading.6
  )
)

(subparagraph
  command:
  _ @module
  toc: (brack_group
    (_) @markup.heading.6
  )?
  text: (curly_group
    (_) @markup.heading.6
  )
)

; Beamer frames
(generic_environment
  (begin
    name: (curly_group_text
      (text) @label
    )
    (#any-of? @label "frame")
  )
  .
  (curly_group
    (_) @markup.heading
  )
)

(
  (generic_command
    command: (command_name) @_name
    arg: (curly_group
      (_) @markup.heading
    )
  )
  (#eq? @_name "\\frametitle")
)

(
  (generic_command
    command: (command_name) @_name
    arg: (curly_group
      (_) @markup.italic
    )
  )
  (#any-of? @_name "\\emph" "\\textit" "\\mathit")
)

(
  (generic_command
    command: (command_name) @_name
    arg: (curly_group
      (_) @markup.strong
    )
  )
  (#any-of? @_name "\\textbf" "\\mathbf")
)

(generic_command
  (command_name) @keyword.conditional
  (#lua-match? @keyword.conditional "^\\if[a-zA-Z@]+$")
)

(generic_command
  (command_name) @keyword.conditional
  (#any-of? @keyword.conditional "\\fi" "\\else")
)

; File inclusion commands
(class_include
  command:
  _ @keyword.import
  path: (curly_group_path) @string
)

(package_include
  command:
  _ @keyword.import
  paths: (curly_group_path_list) @string
)

(latex_include
  command:
  _ @keyword.import
  path: (curly_group_path) @string.special.path
)

(verbatim_include
  command:
  _ @keyword.import
  path: (curly_group_path) @string.special.path
)

(import_include
  command:
  _ @keyword.import
  directory: (curly_group_path) @string.special.path
  file: (curly_group_path) @string.special.path
)

(bibstyle_include
  command:
  _ @keyword.import
  path: (curly_group_path) @string
)

(bibtex_include
  command:
  _ @keyword.import
  paths: (curly_group_path_list) @string.special.path
)

(biblatex_include
  "\\addbibresource" @keyword.import
  glob: (curly_group_glob_pattern) @string.regexp
)

(graphics_include
  command:
  _ @keyword.import
  path: (curly_group_path) @string.special.path
)

(svg_include
  command:
  _ @keyword.import
  path: (curly_group_path) @string.special.path
)

(inkscape_include
  command:
  _ @keyword.import
  path: (curly_group_path) @string.special.path
)

(tikz_library_import
  command:
  _ @keyword.import
  paths: (curly_group_path_list) @string
)

; Math
[
  (displayed_equation)
  (inline_formula)
] @markup.math

(math_environment
  (_) @markup.math
)

; Comments
[
  (line_comment)
  (block_comment)
  (comment_environment)
] @comment

(
  (line_comment) @keyword.directive
  (#lua-match? @keyword.directive "^%% !TeX")
)

(
  (line_comment) @keyword.directive
  (#lua-match? @keyword.directive "^%%&")
)
