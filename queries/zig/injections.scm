;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/zig/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(asm_output_item
  (string) @injection.content
  (#set! injection.language "asm")
)

(asm_input_item
  (string) @injection.content
  (#set! injection.language "asm")
)

(asm_clobbers
  (string) @injection.content
  (#set! injection.language "asm")
)
