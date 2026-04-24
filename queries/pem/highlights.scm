;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-pem/e525b177a229b1154fd81bc0691f943028d9e685/queries/highlights.scm
[
  "BEGIN"
  "END"
] @keyword

(dashes) @punctuation.delimiter

(label) @type

(data) @markup

(comment) @comment
