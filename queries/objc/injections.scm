;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/objc/injections.scm
;; Licensed under the Apache License 2.0
; inherits: c
; TODO(amaanq): uncomment/add when I add asm support
; (ms_asm_block "{" _ @asm "}")
;
; ((asm_specifier (string_literal) @asm)
;   (#offset! @asm 0 1 0 -1))
;
; ((asm_statement (string_literal) @asm)
;   (#offset! @asm 0 1 0 -1))
