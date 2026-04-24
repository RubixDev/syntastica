;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/snakemake/injections.scm
;; Licensed under the Apache License 2.0
; inherits: python
(wildcard
  (constraint) @injection.content
  (#set! injection.language "regex")
)
