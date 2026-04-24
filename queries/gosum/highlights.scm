;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-go-sum/27816eb6b7315746ae9fcf711e4e1396dc1cf237/queries/highlights.scm
[
  "alpha"
  "beta"
  "dev"
  "pre"
  "rc"
  "+incompatible"
] @keyword

(module_path) @string @text.uri

(module_version) @string.special

(hash_version) @attribute

(hash) @symbol

[
  (number)
  (number_with_decimal)
  (hex_number)
] @number

(checksum
  "go.mod" @string
)

[
  ":"
  "."
  "-"
  "/"
] @punctuation.delimiter
