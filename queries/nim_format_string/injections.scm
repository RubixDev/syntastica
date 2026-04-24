;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/nim_format_string/injections.scm
;; Licensed under the Apache License 2.0
(
  (matching_curlies
    (nim_expression
      !escaped_curly
    ) @injection.content
  )
  (#set! injection.language "nim")
)
