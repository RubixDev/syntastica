;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/dockerfile/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (shell_command
    (shell_fragment) @injection.content
  )
  (#set! injection.language "bash")
)

(
  (run_instruction
    (heredoc_block) @injection.content
  )
  (#set! injection.language "bash")
  (#set! injection.include-children)
)
