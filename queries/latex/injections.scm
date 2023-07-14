;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/latex/injections.scm
;; Licensed under the Apache License 2.0
[
  (line_comment)
  (block_comment)
  (comment_environment)
] @comment

(pycode_environment
  code: (source_code) @python
)

(minted_environment
  (begin
    language: (curly_group_text
      (text) @language
    )
  )
  (source_code) @content
)

(
  (generic_environment
    (begin
      name: (curly_group_text
        (text) @_env
      )
    )
  ) @c
  (#any-of? @_env "asy" "asydef")
)
