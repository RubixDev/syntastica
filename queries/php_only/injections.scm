;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/php_only/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "phpdoc")
)

(heredoc
  (heredoc_body) @injection.content
  (heredoc_end) @injection.language
  (#set! injection.include-children)
)

(nowdoc
  (nowdoc_body) @injection.content
  (heredoc_end) @injection.language
  (#set! injection.include-children)
)

; regex
(
  (function_call_expression
    function: (_) @_preg_func_identifier
    arguments: (arguments
      .
      (argument
        (_
          (string_content) @injection.content
        )
      )
    )
  )
  (#set! injection.language "regex")
  (#lua-match? @_preg_func_identifier "^preg_")
)

; bash
(
  (function_call_expression
    function: (_) @_shell_func_identifier
    arguments: (arguments
      .
      (argument
        (_
          (string_content) @injection.content
        )
      )
    )
  )
  (#set! injection.language "bash")
  (#any-of?
    @_shell_func_identifier
    "shell_exec"
    "escapeshellarg"
    "escapeshellcmd"
    "exec"
    "passthru"
    "proc_open"
    "shell_exec"
    "system"
  )
)

(expression_statement
  (shell_command_expression
    (string_content) @injection.content
  )
  (#set! injection.language "bash")
)
