;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/cpp/injections.scm
;; Licensed under the Apache License 2.0
; inherits: c
(preproc_arg) @cpp

(comment) @comment

(raw_string_literal
  delimiter: (raw_string_delimiter) @language
  (raw_string_content) @content
)
