;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/tlaplus/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (block_comment_text)
  ] @injection.content
  (#set! injection.language "comment")
)
