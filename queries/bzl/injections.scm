;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/starlark/injections.scm
;; Licensed under the Apache License 2.0
(
  (binary_operator
    left: (string
      (string_content) @injection.content
    )
    operator:
    "%"
  )
  (#set! injection.language "printf")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
