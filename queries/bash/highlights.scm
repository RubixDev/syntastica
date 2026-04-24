;; Forked from https://raw.githubusercontent.com/tree-sitter/tree-sitter-bash/a06c2e4415e9bc0346c6b86d401879ffb44058f7/queries/highlights.scm
[
  (string)
  (raw_string)
  (heredoc_body)
  (heredoc_start)
] @string

(command_name) @function

(variable_name) @property

[
  "case"
  "do"
  "done"
  "elif"
  "else"
  "esac"
  "export"
  "fi"
  "for"
  "function"
  "if"
  "in"
  "select"
  "then"
  "unset"
  "until"
  "while"
] @keyword

(comment) @comment

(function_definition
  name: (word) @function
)

(file_descriptor) @number

[
  (command_substitution)
  (process_substitution)
  (expansion)
] @embedded

[
  "$"
  "&&"
  ">"
  ">>"
  "<"
  "|"
] @operator

(
  (command
    (_) @constant
  )
  (#match? @constant "^-")
)
