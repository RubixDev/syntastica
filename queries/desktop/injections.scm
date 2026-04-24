;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/desktop/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (entry
    key: (identifier) @_exec
    value: (string) @injection.content
  )
  (#eq? @_exec "Exec")
  (#set! injection.language "bash")
)
