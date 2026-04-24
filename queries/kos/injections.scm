;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/kos/injections.scm
;; Licensed under the Apache License 2.0
(
  (
    (comment) @_jsdoc_comment
    (#lua-match? @_jsdoc_comment "^/[*][*][^*].*[*]/$")
  ) @injection.content
  (#set! injection.language "jsdoc")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
