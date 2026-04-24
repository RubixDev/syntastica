;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/kdl/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (single_line_comment)
    (multi_line_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
