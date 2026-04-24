;; Forked from https://raw.githubusercontent.com/urbit-pilled/tree-sitter-hoon/1545137aadcc63660c47db9ad98d02fa602655d0/queries/highlights.scm
(number) @number

(string) @string

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  (coreTerminator)
  (seriesTerminator)
] @punctuation.delimiter

(rune) @keyword

(term) @constant

(aura) @constant.builtin

(Gap) @comment

(boolean) @constant.builtin

(date) @constant.builtin

(mold) @constant.builtin

(specialIndex) @constant.builtin

(lark) @operator

(fullContext) @constant.builtin
