;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/scala/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (block_comment) @injection.content
  (#set! injection.language "comment")
)
