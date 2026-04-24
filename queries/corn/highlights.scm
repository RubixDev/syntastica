;; Forked from https://raw.githubusercontent.com/corn-config/tree-sitter-corn/464654742cbfd3a3de560aba120998f1d5dfa844/queries/highlights.scm
"let" @keyword

"in" @keyword

[
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

(path_seg) @string.special.key

"." @punctuation.delimiter

(input) @constant

(comment) @comment

(string) @string

(char) @string

(integer) @number

(float) @float

(boolean) @boolean

(null) @keyword

(ERROR) @error
