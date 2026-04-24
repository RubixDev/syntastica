;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/systemtap/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (embedded_code) @injection.content
  (#set! injection.language "c")
)
