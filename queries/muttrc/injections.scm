;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/muttrc/injections.scm
;; Licensed under the Apache License 2.0
(
  (regex) @injection.content
  (#set! injection.language "regex")
)

(
  (shell) @injection.content
  (#set! injection.language "bash")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
