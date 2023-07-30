;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/jsx/highlights.scm
;; Licensed under the Apache License 2.0
(jsx_element
  open_tag: (jsx_opening_element
    [
      "<"
      ">"
    ] @tag.delimiter
  )
)

(jsx_element
  close_tag: (jsx_closing_element
    [
      ; crates.io skip
      "</"
      ; non-crates.io skip
      "<"
      ; non-crates.io skip
      "/"
      ">"
    ] @tag.delimiter
  )
)

(jsx_self_closing_element
  [
    "<"
    ; crates.io skip
    "/>"
    ; non-crates.io skip
    "/"
    ; non-crates.io skip
    ">"
  ] @tag.delimiter
)

(jsx_attribute
  (property_identifier) @tag.attribute
)

(jsx_opening_element
  name: (identifier) @tag
)

(jsx_closing_element
  name: (identifier) @tag
)

(jsx_self_closing_element
  name: (identifier) @tag
)

(jsx_opening_element
  (
    (identifier) @constructor
    (#lua-match? @constructor "^[A-Z]")
  )
)

; Handle the dot operator effectively - <My.Component>
(jsx_opening_element
  ; crates.io skip
  (
    (member_expression
      (identifier) @tag
      (property_identifier) @constructor
    )
  )
  ; non-crates.io skip
  (
    (nested_identifier
      (identifier) @tag
      (identifier) @constructor
    )
  )
)

(jsx_closing_element
  (
    (identifier) @constructor
    (#lua-match? @constructor "^[A-Z]")
  )
)

; Handle the dot operator effectively - </My.Component>
(jsx_closing_element
  ; crates.io skip
  (
    (member_expression
      (identifier) @tag
      (property_identifier) @constructor
    )
  )
  ; non-crates.io skip
  (
    (nested_identifier
      (identifier) @tag
      (identifier) @constructor
    )
  )
)

(jsx_self_closing_element
  (
    (identifier) @constructor
    (#lua-match? @constructor "^[A-Z]")
  )
)

; Handle the dot operator effectively - <My.Component />
(jsx_self_closing_element
  ; crates.io skip
  (
    (member_expression
      (identifier) @tag
      (property_identifier) @constructor
    )
  )
  ; non-crates.io skip
  (
    (nested_identifier
      (identifier) @tag
      (identifier) @constructor
    )
  )
)

(jsx_text) @none
