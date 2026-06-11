;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gleam/injections.scm
;; Licensed under the Apache License 2.0
; Comments
(
  [
    (module_comment)
    (statement_comment)
    (comment)
  ] @injection.content
  (#set! injection.language "comment")
)
