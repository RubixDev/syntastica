;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/yang/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (statement
    (statement_keyword
      "pattern"
    )
    (argument
      (string) @injection.content
    )
  )
  (#set! injection.language "regex")
  (#offset! @injection.content 0 1 0 -1)
)
