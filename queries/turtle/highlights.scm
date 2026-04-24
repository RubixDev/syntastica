;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/turtle/highlights.scm
;; Licensed under the Apache License 2.0
(string) @string

(lang_tag) @type

[
  "_:"
  "<"
  ">"
  (namespace)
] @module

[
  (iri_reference)
  (prefixed_name)
] @variable

(blank_node_label) @variable

"a" @variable.builtin

(integer) @number

[
  (decimal)
  (double)
] @number.float

(boolean_literal) @boolean

[
  "BASE"
  "PREFIX"
  "@prefix"
  "@base"
] @keyword

[
  "."
  ","
  ";"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  (anon)
] @punctuation.bracket

(comment) @comment @spell

(echar) @string.escape

(rdf_literal
  "^^" @type
  datatype: (_
    [
      "<"
      ">"
      (namespace)
    ] @type
  ) @type
)
