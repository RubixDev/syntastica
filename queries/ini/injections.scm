;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ini/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment
    (text) @injection.content
  )
  (#set! injection.language "comment")
)
