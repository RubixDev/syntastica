;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/markdown_inline/highlights.scm
;; Licensed under the Apache License 2.0
(code_span) @markup.raw

(emphasis) @markup.italic

(strong_emphasis) @markup.strong

(strikethrough) @markup.strikethrough

[
  (backslash_escape)
  (hard_line_break)
] @string.escape

; Conceal inline links
(inline_link
  [
    "["
    "]"
    "("
    (link_destination)
    ")"
  ] @markup.link
)

[
  (link_label)
  (link_text)
  (link_title)
  (image_description)
] @markup.link.label

(
  (inline_link
    (link_destination) @_url
  ) @_label
)

(
  (image
    (link_destination) @_url
  ) @_label
)

; Conceal image links
(image
  [
    "!"
    "["
    "]"
    "("
    (link_destination)
    ")"
  ] @markup.link
)

; Conceal full reference links
(full_reference_link
  [
    "["
    "]"
    (link_label)
  ] @markup.link
)

; Conceal collapsed reference links
(collapsed_reference_link
  [
    "["
    "]"
  ] @markup.link
)

; Conceal shortcut links
(shortcut_link
  [
    "["
    "]"
  ] @markup.link
)

[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @markup.link.url

(
  (uri_autolink) @_url
  (#offset! @_url 0 1 0 -1)
)

; Replace common HTML entities.
(
  (entity_reference) @character.special
  (#eq? @character.special "&nbsp;")
)

(
  (entity_reference) @character.special
  (#eq? @character.special "&lt;")
)

(
  (entity_reference) @character.special
  (#eq? @character.special "&gt;")
)

(
  (entity_reference) @character.special
  (#eq? @character.special "&amp;")
)

(
  (entity_reference) @character.special
  (#eq? @character.special "&quot;")
)

(
  (entity_reference) @character.special
  (#any-of? @character.special "&ensp;" "&emsp;")
)
