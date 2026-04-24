;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/d/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (call_expression
    (type) @_printf
    (named_arguments
      "("
      .
      (named_argument
        (expression
          (string_literal) @injection.content
        )
      )
    )
  )
  (#eq? @_printf "printf")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "printf")
)

; TODO: uncomment when asm is added
; ((asm_inline) @injection.content
;   (#set! injection.language "asm")
;   (#set! injection.combined))
