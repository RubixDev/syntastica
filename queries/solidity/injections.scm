;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/solidity/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (comment) @injection.content
  (#lua-match? @injection.content "^///[^/]")
  (#set! injection.language "doxygen")
)

(
  (comment) @injection.content
  (#lua-match? @injection.content "^///$")
  (#set! injection.language "doxygen")
)

(
  (comment) @injection.content
  (#lua-match? @injection.content "^/[*][*][^*].*[*]/$")
  (#set! injection.language "doxygen")
)
