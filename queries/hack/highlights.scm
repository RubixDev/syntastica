;; Forked from https://raw.githubusercontent.com/slackhq/tree-sitter-hack/1a7ded90288189746c54861ac144ede97df95081/queries/highlights.scm
(comment) @comment

(string) @string

(heredoc) @string

(prefixed_string) @string

[
  "class"
  "interface"
  "trait"
  "public"
  "protected"
  "private"
  "static"
  "async"
  "function"
  "return"
  "if"
  "else"
  "elseif"
  "while"
  "for"
  "foreach"
  "break"
  "continue"
  "type"
  "new"
  "throw"
] @keyword

(type_specifier) @type
