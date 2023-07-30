;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/c/injections.scm
;; Licensed under the Apache License 2.0
(preproc_function_def
  (preproc_arg) @c
)

(preproc_call
  (preproc_arg) @c
)

(comment) @comment

(gnu_asm_expression
  assembly_code: (string_literal) @asm
)

(gnu_asm_expression
  assembly_code: (concatenated_string
    (string_literal) @asm
  )
)
