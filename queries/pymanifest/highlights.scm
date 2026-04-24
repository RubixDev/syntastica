;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-pymanifest/debbdb83fe6356adc7261c41c69b45ba49c97294/queries/highlights.scm
(keyword) @keyword

(dir_sep) @punctuation.delimiter

(glob) @punctuation.special

(linebreak) @escape

(char_sequence) @string.special

(char_sequence
  [
    "["
    "]"
  ] @punctuation.bracket
)

(char_sequence
  "!" @operator
)

(char_range
  "-" @operator
)

(escaped_char) @string.escape

(ERROR) @error

(comment) @comment
