;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/jsdoc/highlights.scm
;; Licensed under the Apache License 2.0
(tag_name) @keyword

(type) @type

[
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

[
  ":"
  "."
  "#"
  "~"
] @punctuation.delimiter

(path_expression
  "/" @punctuation.delimiter
)

(identifier) @variable

(tag
  (tag_name) @_name
  (identifier) @function
  (#any-of? @_name "@callback" "@function" "@func" "@method")
)

(tag
  (tag_name) @_name
  (identifier) @variable.parameter
  (#any-of? @_name "@param" "@arg" "@argument")
)

(tag
  (tag_name) @_name
  (identifier) @property
  (#any-of? @_name "@prop" "@property")
)

(tag
  (tag_name) @_name
  (identifier) @type
  (#eq? @_name "@typedef")
)
