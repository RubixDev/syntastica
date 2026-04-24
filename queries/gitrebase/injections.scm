;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/git_rebase/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (operation
    (command) @_command
    (message) @injection.content
  )
  (#set! injection.language "bash")
  (#any-of? @_command "exec" "x")
)
