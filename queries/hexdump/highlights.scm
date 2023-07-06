;; Forked from https://github.com/rush-rs/tree-sitter-hexdump/blob/09eaf4fcfed00be93928d7d3d82b490cd1343b80/queries/hexdump/highlights.scm
;; Licensed under the MIT license
(line_comment) @comment

(int) @number

(string) @string

":" @punctuation.delimiter
