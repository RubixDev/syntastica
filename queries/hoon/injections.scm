;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hoon/injections.scm
;; Licensed under the Apache License 2.0
(
  (lineComment) @injection.content
  (#set! injection.language "comment")
)
