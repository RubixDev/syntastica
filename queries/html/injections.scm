;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/html/injections.scm
;; Licensed under the Apache License 2.0
; inherits html_tags
(element
  (start_tag
    (tag_name) @_py_script
  )
  (text) @python
  (#any-of? @_py_script "py-script" "py-repl")
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
  (raw_text) @python
  (#eq? @_attr "type")
  ; not adding type="py" here as it's handled by html_tags
  (#any-of? @_type "pyscript" "py-script")
)

(element
  (start_tag
    (tag_name) @_py_config
  )
  (text) @toml
  (#eq? @_py_config "py-config")
)
