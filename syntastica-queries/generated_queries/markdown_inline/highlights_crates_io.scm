(shortcut_link
  [
    "["
    "]"
  ] @conceal
  (#set! conceal "")
)

(collapsed_reference_link
  [
    "["
    "]"
  ] @conceal
  (#set! conceal "")
)

(full_reference_link
  [
    "["
    "]"
    (link_label)
  ] @conceal
  (#set! conceal "")
)

(image
  [
    "!"
    "["
    "]"
    "("
    (link_destination)
    ")"
  ] @conceal
  (#set! conceal "")
)

(inline_link
  [
    "["
    "]"
    "("
    (link_destination)
    ")"
  ] @conceal
  (#set! conceal "")
)

(
  [
    (code_span_delimiter)
    (emphasis_delimiter)
  ] @conceal
  (#set! conceal "")
)

(shortcut_link
  [
    "["
    "]"
  ] @punctuation.bracket
)

(inline_link
  [
    "["
    "]"
    "("
    ")"
  ] @punctuation.bracket
)

(image
  [
    "["
    "]"
    "("
    ")"
  ] @punctuation.bracket
)

(image
  "!" @punctuation.special
)

[
  (backslash_escape)
  (hard_line_break)
] @string.escape

[
  (link_label)
  (link_text)
  (image_description)
] @text.reference

(shortcut_link
  (link_text) @nospell
)

[
  (link_destination)
  (uri_autolink)
] @text.uri @nospell

(strikethrough) @text.strike

(strong_emphasis) @text.strong

(emphasis) @text.emphasis

[
  (emphasis_delimiter)
  (code_span_delimiter)
] @punctuation.delimiter

[
  (code_span)
  (link_title)
] @text.literal @nospell
