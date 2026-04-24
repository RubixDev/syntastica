;; Forked from https://raw.githubusercontent.com/sogaiu/tree-sitter-clojure/e43eff80d17cf34852dcd92ca5e6986d23a7040f/queries/highlights.scm
;; Literals
(num_lit) @number

[
  (char_lit)
  (str_lit)
] @string

[
  (bool_lit)
  (nil_lit)
] @constant.builtin

(kwd_lit) @constant

;; Comments
(comment) @comment

;; Treat quasiquotation as operators for the purpose of highlighting.
[
  "'"
  "`"
  "~"
  "@"
  "~@"
] @operator
