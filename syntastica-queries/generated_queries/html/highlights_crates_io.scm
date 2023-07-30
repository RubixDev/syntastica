"<!" @tag.delimiter

(doctype) @constant

"=" @operator

[
  "<"
  ">"
  "</"
  "/>"
] @tag.delimiter

(
  (attribute
    (attribute_name) @_attr
    (quoted_attribute_value
      (attribute_value) @text.uri
    )
  )
  (#match? @_attr "^(href|src)$")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.uri
  )
  (#eq? @_tag "a")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.literal
  )
  (#match? @_tag "^(code|kbd)$")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.underline
  )
  (#eq? @_tag "u")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.strike
  )
  (#match? @_tag "^(s|del)$")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.emphasis
  )
  (#match? @_tag "^(em|i)$")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.strong
  )
  (#match? @_tag "^(strong|b)$")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.title.6
  )
  (#eq? @_tag "h6")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.title.5
  )
  (#eq? @_tag "h5")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.title.4
  )
  (#eq? @_tag "h4")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.title.3
  )
  (#eq? @_tag "h3")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.title.2
  )
  (#eq? @_tag "h2")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.title.1
  )
  (#eq? @_tag "h1")
)

(
  (element
    (start_tag
      (tag_name) @_tag
    )
    (text) @text.title
  )
  (#eq? @_tag "title")
)

(text) @text @spell

(attribute
  (quoted_attribute_value) @string
)

(attribute_name) @tag.attribute

(comment) @comment

(erroneous_end_tag_name) @error

(tag_name) @tag
