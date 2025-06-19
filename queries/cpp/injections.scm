;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/cpp/injections.scm
;; Licensed under the Apache License 2.0
(
  (preproc_arg) @injection.content
  (#set! injection.language "cpp")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (comment) @injection.content
  (#lua-match? @injection.content "/[*/][!*/]<?[^a-zA-Z]")
  (#set! injection.language "doxygen")
)

(raw_string_literal
  delimiter: (raw_string_delimiter) @injection.language
  (raw_string_content) @injection.content
)
