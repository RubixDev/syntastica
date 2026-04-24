;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/eex/injections.scm
;; Licensed under the Apache License 2.0
; EEx expressions are Elixir
(
  (expression) @injection.content
  (#set! injection.language "elixir")
)

; EEx expressions can span multiple interpolated lines
(
  (partial_expression) @injection.content
  (#set! injection.language "elixir")
  (#set! injection.combined)
)
