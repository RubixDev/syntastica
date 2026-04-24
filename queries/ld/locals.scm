;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/linkerscript/locals.scm
;; Licensed under the Apache License 2.0
; References
[
  (symbol)
  (filename)
  (quoted_symbol)
] @local.reference

; Definitions
(output_section
  name: (symbol) @local.definition.var
)

(memory_command
  name: (symbol) @local.definition.var
)

(phdrs_command
  name: (symbol) @local.definition.var
)
