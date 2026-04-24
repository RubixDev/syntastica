;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gitcommit/injections.scm
;; Licensed under the Apache License 2.0
(
  (diff) @injection.content
  (#set! injection.language "diff")
)

(
  (rebase_command) @injection.content
  (#set! injection.language "git_rebase")
)
