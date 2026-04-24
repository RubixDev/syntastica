;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/inko/injections.scm
;; Licensed under the Apache License 2.0
(
  (line_comment) @injection.content
  (#set! injection.language "comment")
)
