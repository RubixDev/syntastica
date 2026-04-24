;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/dtd/injections.scm
;; Licensed under the Apache License 2.0
(
  (Comment) @injection.content
  (#set! injection.language "comment")
)
