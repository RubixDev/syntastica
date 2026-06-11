;; Forked from https://github.com/rush-rs/tree-sitter-hexdump/blob/main/queries/hexdump/highlights.scm
;; Licensed under the MIT license
(line_comment) @comment

(int) @number

(string) @string

":" @punctuation.delimiter
