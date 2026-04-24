;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/awk/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (regex) @injection.content
  (#set! injection.language "regex")
)

(
  (print_statement
    (exp_list
      .
      (string) @injection.content
    )
  )
  (#set! injection.language "printf")
)

(
  (printf_statement
    (exp_list
      .
      (string) @injection.content
    )
  )
  (#set! injection.language "printf")
)
