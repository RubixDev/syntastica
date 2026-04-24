;; Forked from https://raw.githubusercontent.com/PorterAtGoogle/tree-sitter-textproto/568471b80fd8793d37ed01865d8c2208a9fefd1b/queries/highlights.scm
(string) @string

(field_name) @attribute

(comment) @comment

(number) @number

; For stuff like "inf" and "-inf".
(scalar_value
  (identifier)
) @number

(scalar_value
  (signed_identifier)
) @number

(open_squiggly) @punctuation.bracket

(close_squiggly) @punctuation.bracket

(open_square) @punctuation.bracket

(close_square) @punctuation.bracket

(open_arrow) @punctuation.bracket

(close_arrow) @punctuation.bracket
