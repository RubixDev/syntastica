;; Forked from https://raw.githubusercontent.com/sogaiu/tree-sitter-janet-simple/7e28cbf1ca061887ea43591a2898001f4245fddf/queries/highlights.scm
(num_lit) @number

[
  (buf_lit)
  (long_buf_lit)
  (long_str_lit)
  (str_lit)
] @string

[
  (bool_lit)
  (nil_lit)
] @constant.builtin

(kwd_lit) @constant

(comment) @comment

;; Treat quasiquotation as operators for the purpose of highlighting.
[
  "'"
  "~"
  ","
] @operator
