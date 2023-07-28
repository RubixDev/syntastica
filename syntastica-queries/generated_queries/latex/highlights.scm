(command_name) @nospell

(citation
  keys:
  _ @nospell
)

(generic_environment
  begin:
  _ @nospell
  end:
  _ @nospell
)

(key_value_pair) @nospell

(displayed_equation) @nospell

(inline_formula) @nospell

(text) @spell

(tikz_library_import
  command:
  _ @include
  paths: (curly_group_path_list) @string
)

(graphics_include
  command:
  _ @include
  path: (curly_group_path) @string
)

(biblatex_include
  "\\addbibresource" @include
  glob: (curly_group_glob_pattern) @string.regex
)

(bibtex_include
  command:
  _ @include
  path: (curly_group_path) @string
)

(import_include
  command:
  _ @include
  directory: (curly_group_path) @string
  file: (curly_group_path) @string
)

(latex_include
  command:
  _ @include
  path: (curly_group_path) @string
)

(package_include
  command:
  _ @include
  paths: (curly_group_path_list) @string
)

(class_include
  command:
  _ @include
  path: (curly_group_path) @string
)

(
  (generic_command
    command: (command_name) @_name
    .
    arg: (curly_group
      (_) @text.uri
    )
  )
  (#match? @_name "^(\\\\url|\\\\href)$")
)

(
  (generic_command
    command: (command_name) @_name
    arg: (curly_group
      (_) @text.strong
    )
  )
  (#match? @_name "^(\\\\textbf|\\\\mathbf)$")
)

(
  (generic_command
    command: (command_name) @_name
    arg: (curly_group
      (_) @text.emphasis
    )
  )
  (#match? @_name "^(\\\\textit|\\\\mathit)$")
)

(
  (generic_command
    command: (command_name) @_name
    arg: (curly_group
      (_) @text.emphasis
    )
  )
  (#eq? @_name "\\emph")
)

(
  (generic_command
    command: (command_name) @_name
    arg: (curly_group
      (text) @text.title
    )
  )
  (#eq? @_name "\\frametitle")
)

(generic_environment
  (begin
    name: (curly_group_text
      (text) @text.environment.name
    )
    (#match? @text.environment.name "^(frame)$")
  )
  .
  (curly_group
    (_) @text.title
  )
)

(subparagraph
  command:
  _ @namespace
  toc: (brack_group
    (_) @text.title.6
  )?
  text: (curly_group
    (_) @text.title.6
  )
)

(paragraph
  command:
  _ @namespace
  toc: (brack_group
    (_) @text.title.6
  )?
  text: (curly_group
    (_) @text.title.6
  )
)

(subsubsection
  command:
  _ @namespace
  toc: (brack_group
    (_) @text.title.5
  )?
  text: (curly_group
    (_) @text.title.5
  )
)

(subsection
  command:
  _ @namespace
  toc: (brack_group
    (_) @text.title.4
  )?
  text: (curly_group
    (_) @text.title.4
  )
)

(section
  command:
  _ @namespace
  toc: (brack_group
    (_) @text.title.3
  )?
  text: (curly_group
    (_) @text.title.3
  )
)

(part
  command:
  _ @namespace
  toc: (brack_group
    (_) @text.title.2
  )?
  text: (curly_group
    (_) @text.title.2
  )
)

(chapter
  command:
  _ @namespace
  toc: (brack_group
    (_) @text.title.2
  )?
  text: (curly_group
    (_) @text.title.2
  )
)

(author_declaration
  command:
  _ @namespace
  authors: (curly_group_author_list
    (
      (author)+ @text.title.1
    )
  )
)

(title_declaration
  command:
  _ @namespace
  options: (brack_group
    (_) @text.title.1
  )?
  text: (curly_group
    (_) @text.title.1
  )
)

(math_environment
  (end
    command:
    _ @text.math
    name: (curly_group_text
      (text) @text.math
    )
  )
)

(math_environment
  (text) @text.math
)

(math_environment
  (begin
    command:
    _ @text.math
    name: (curly_group_text
      (text) @text.math
    )
  )
)

[
  (displayed_equation)
  (inline_formula)
] @text.math

(color_reference
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(color_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(acronym_reference
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(acronym_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(glossary_entry_reference
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(glossary_entry_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(citation
  command:
  _ @function.macro
  keys: (curly_group_text_list) @text.reference
)

(label_number
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
  number: (_) @text.reference
)

(label_reference
  command:
  _ @function.macro
  names: (curly_group_text_list
    (_) @text.reference
  )
)

(label_reference_range
  command:
  _ @function.macro
  from: (curly_group_text
    (_) @text.reference
  )
  to: (curly_group_text
    (_) @text.reference
  )
)

(label_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(paired_delimiter_definition
  command:
  _ @function.macro
  declaration: (curly_group_command_name
    (_) @function
  )
)

(theorem_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.environment.name
  )
)

(environment_definition
  command:
  _ @function.macro
  name: (curly_group_text
    (_) @text.reference
  )
)

(let_command_definition
  command:
  _ @function.macro
  declaration: (_) @function
)

(old_command_definition
  command:
  _ @function.macro
  declaration: (_) @function
)

(new_command_definition
  command:
  _ @function.macro
  declaration: (curly_group_command_name
    (_) @function
  )
)

(end
  command:
  _ @text.environment
  name: (curly_group_text
    (text) @text.environment.name
  )
)

(begin
  command:
  _ @text.environment
  name: (curly_group_text
    (text) @text.environment.name
  )
)

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(
  (word) @punctuation.delimiter
  (#eq? @punctuation.delimiter "&")
)

"\\item" @punctuation.special

[
  (operator)
  "="
] @operator

[
  (brack_group)
  (brack_group_argc)
] @parameter

(
  (line_comment) @preproc
  (#match? @preproc "^% !TeX")
)

[
  (line_comment)
  (block_comment)
  (comment_environment)
] @comment

(key_value_pair
  key: (_) @parameter
  value: (_)
)

(caption
  command:
  _ @function
)

(command_name) @function

(ERROR) @error
