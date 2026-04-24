;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/idl/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#lua-match? @injection.content "/[*/][!*/]<?[^a-zA-Z]")
  (#set! injection.language "doxygen")
)

(
  (comment) @injection.content
  (#not-lua-match? @injection.content "/[*/][!*/]<?[^a-zA-Z]")
  (#not-lua-match? @injection.content "//@[a-zA-Z]")
  (#set! injection.language "comment")
)
