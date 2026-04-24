;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hare/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (call_expression
    .
    (_) @_fnname
    .
    "("
    .
    (_
      [
        (string_content)
        (raw_string_content)
      ] @injection.content
    )
    .
    ")"
  )
  (#any-of? @_fnname "compile" "regex::compile")
  (#set! injection.language "regex")
)
