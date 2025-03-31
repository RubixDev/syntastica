;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/make/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (shell_text) @injection.content
  (#set! injection.language "bash")
)

(
  (shell_command) @injection.content
  (#set! injection.language "bash")
)
