;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/asm/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
