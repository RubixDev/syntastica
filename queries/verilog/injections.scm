;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/verilog/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (macro_text) @injection.content
  (#set! injection.language "verilog")
)
