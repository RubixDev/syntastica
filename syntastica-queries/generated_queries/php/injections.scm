(
  (text) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)

(
  (comment) @injection.content
  (#set! injection.language "phpdoc")
)

(
  (function_call_expression
    function: (_) @_preg_func_identifier
    arguments: (arguments
      .
      (argument
        (_
          (string_value) @injection.content
          (#set! injection.language "regex")
        )
      )
    )
  )
  (#match? @_preg_func_identifier "^preg_")
)

(
  (function_call_expression
    function: (_) @_shell_func_identifier
    arguments: (arguments
      .
      (argument
        (_
          (string_value) @injection.content
          (#set! injection.language "bash")
        )
      )
    )
  )
  (#match? @_shell_func_identifier "^(shell_exec|escapeshellarg|escapeshellcmd|exec|passthru|proc_open|shell_exec|system)$")
)

(expression_statement
  (shell_command_expression
    (string_value) @injection.content
    (#set! injection.language "bash")
  )
)
