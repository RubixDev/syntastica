;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/jsx/injections.scm
;; Licensed under the Apache License 2.0
; Styled Jsx <style jsx>
(jsx_element
  (jsx_opening_element
    (identifier) @_name
    (#eq? @_name "style")
    (jsx_attribute) @_attr
    (#eq? @_attr "jsx")
  )
  (jsx_expression
    (
      (template_string) @injection.content
      (#set! injection.language "css")
    )
    (#offset! @injection.content 0 1 0 -1)
  )
)
