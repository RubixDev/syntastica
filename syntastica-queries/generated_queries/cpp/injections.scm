(preproc_function_def
  (preproc_arg) @injection.content
  (#set! injection.language "c")
)

(preproc_call
  (preproc_arg) @injection.content
  (#set! injection.language "c")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(gnu_asm_expression
  assembly_code: (string_literal) @injection.content
  (#set! injection.language "asm")
)

(gnu_asm_expression
  assembly_code: (concatenated_string
    (string_literal) @injection.content
    (#set! injection.language "asm")
  )
)

(
  (preproc_arg) @injection.content
  (#set! injection.language "cpp")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(raw_string_literal
  delimiter: (raw_string_delimiter) @injection.language
  (raw_string_content) @injection.content
)
