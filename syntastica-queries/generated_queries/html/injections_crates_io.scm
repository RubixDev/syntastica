(
  (style_element
    (start_tag) @_no_type_lang
    (#not-match? @_no_type_lang "[ \\t\\n\\v\\f\\r]lang[ \\t\\n\\v\\f\\r]*=")
    (#not-match? @_no_type_lang "[ \\t\\n\\v\\f\\r]type[ \\t\\n\\v\\f\\r]*=")
    (raw_text) @injection.content
  )
  (#set! injection.language "css")
)

(
  (style_element
    (start_tag
      (attribute
        (attribute_name) @_type
        (quoted_attribute_value
          (attribute_value) @_css
        )
      )
    )
    (raw_text) @injection.content
  )
  (#eq? @_type "type")
  (#eq? @_css "text/css")
  (#set! injection.language "css")
)

(
  (script_element
    (start_tag) @_no_type_lang
    (#not-match? @_no_type_lang "[ \\t\\n\\v\\f\\r]lang[ \\t\\n\\v\\f\\r]*=")
    (#not-match? @_no_type_lang "[ \\t\\n\\v\\f\\r]type[ \\t\\n\\v\\f\\r]*=")
    (raw_text) @injection.content
  )
  (#set! injection.language "javascript")
)

(script_element
  (start_tag
    (
      (attribute
        (attribute_name) @_attr
        (#eq? @_attr "type")
        (quoted_attribute_value
          (attribute_value) @injection.language
        )
      )
    )
  )
  (raw_text) @injection.content
)

(
  (attribute
    (attribute_name) @_attr
    (quoted_attribute_value
      (attribute_value) @injection.content
    )
  )
  (#eq? @_attr "style")
  (#set! injection.language "css")
)

(
  (attribute
    (quoted_attribute_value
      (attribute_value) @injection.content
    )
  )
  (#match? @injection.content "\\$\\{")
  (#offset! @injection.content 0 2 0 -1)
  (#set! injection.language "javascript")
)

(
  (attribute
    (attribute_value) @injection.content
  )
  (#match? @injection.content "\\$\\{")
  (#offset! @injection.content 0 2 0 -2)
  (#set! injection.language "javascript")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(element
  (_
    (tag_name) @_tagname
    (#eq? @_tagname "input")
    (
      (attribute
        (attribute_name) @_attr
        [
          (quoted_attribute_value
            (attribute_value) @injection.content
          )
          (attribute_value) @injection.content
        ]
        (#eq? @_attr "pattern")
        (#set! injection.language "regex")
      )
    )
  )
)

(attribute
  (attribute_name) @_name
  (#match? @_name "^on[a-z]+$")
  (quoted_attribute_value
    (attribute_value) @injection.content
  )
  (#set! injection.language "javascript")
)

(element
  (start_tag
    (tag_name) @_py_script
  )
  (text) @injection.content
  (#match? @_py_script "^(py-script|py-repl)$")
  (#set! injection.language "python")
)

(script_element
  (start_tag
    (attribute
      (attribute_name) @_attr
      (quoted_attribute_value
        (attribute_value) @_type
      )
    )
  )
  (raw_text) @injection.content
  (#eq? @_attr "type")
  (#match? @_type "^(pyscript|py-script)$")
  (#set! injection.language "python")
)

(element
  (start_tag
    (tag_name) @_py_config
  )
  (text) @injection.content
  (#eq? @_py_config "py-config")
  (#set! injection.language "toml")
)
