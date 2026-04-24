;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/pug/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (javascript) @injection.content
  (#set! injection.language "javascript")
)

(
  (attribute_name) @_attribute_name
  (quoted_attribute_value
    (attribute_value) @injection.content
    (#set! injection.language "javascript")
  )
  (#match? @_attribute_name "^(:|v-bind|v-|\\@)")
)
