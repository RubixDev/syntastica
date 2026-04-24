;; Forked from https://raw.githubusercontent.com/glapa-grossklag/tree-sitter-elsa/0a66b2b3f3c1915e67ad2ef9f7dbd2a84820d9d7/queries/highlights.scm
; Keywords
[
  "eval"
  "let"
] @keyword

; Function
(function) @function

; Method
(method) @method

; Parameter
(parameter) @parameter

; Variables
(identifier) @variable

; Operators
[
  "\\"
  "->"
  "="
  (step)
] @operator

; Punctuation
[
  "("
  ")"
] @punctuation.bracket

":" @punctuation.delimiter

; Comments
(comment) @comment
