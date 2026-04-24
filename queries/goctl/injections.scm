;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/goctl/injections.scm
;; Licensed under the Apache License 2.0
; Inject comment language for goctl
(
  (comment) @injection.content
  (#set! injection.language "comment")
)
