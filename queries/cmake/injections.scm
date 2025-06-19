;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/cmake/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (bracket_comment)
    (line_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
