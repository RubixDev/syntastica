;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/tmux/injections.scm
;; Licensed under the Apache License 2.0
(
  (shell) @injection.content
  (#set! injection.language "bash")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
