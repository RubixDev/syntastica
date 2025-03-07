;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/typst/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(raw_blck
  (ident) @injection.language
  (blob) @injection.content
)
