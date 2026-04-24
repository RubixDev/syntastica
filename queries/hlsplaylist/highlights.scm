;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hlsplaylist/highlights.scm
;; Licensed under the Apache License 2.0
; Comments
(comment) @comment @spell

; General
(uri) @string.special.url

(tag_name) @keyword

(attribute_name) @attribute

[
  (dec)
  (hex)
  (resolution)
  (range)
] @number

(float) @number.float

(string) @string

[
  (enum)
  (date_time_msec)
] @string.special

(title) @markup.heading

; Literals
[
  "="
  "x"
  "@"
] @operator

[
  ":"
  ","
] @punctuation.delimiter

"#" @punctuation.special
