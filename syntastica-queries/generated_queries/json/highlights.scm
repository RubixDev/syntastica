(
  (escape_sequence) @conceal
  (#eq? @conceal "\\\"")
  (#set! conceal "\"")
)

(
  (escape_sequence) @string.escape
)

(
  ("\""
    @conceal
  )
  (#set! conceal "")
)

(
  [
    "["
    "]"
    "{"
    "}"
  ] @punctuation.bracket
)

(
  [
    ","
    ":"
  ] @punctuation.delimiter
)

(
  (ERROR) @error
)

(
  (string_content) @spell
)

(array
  (string) @string
)

(pair
  value: (string) @string
)

(pair
  key: (string) @label
)

(
  (number) @number
)

(
  (null) @constant.builtin
)

(
  [
    (true)
    (false)
  ] @boolean
)
