;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/fsh/injections.scm
;; Licensed under the Apache License 2.0
(
  (fsh_comment) @injection.content
  (#set! injection.language "comment")
)
