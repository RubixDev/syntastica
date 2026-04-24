;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/templ/injections.scm
;; Licensed under the Apache License 2.0
; inherits: go
(
  (element_comment) @injection.content
  (#set! injection.language "comment")
)

(
  (script_block_text) @injection.content
  (#set! injection.language "javascript")
)

(
  (script_element_text) @injection.content
  (#set! injection.language "javascript")
)

(
  (style_element_text) @injection.content
  (#set! injection.language "css")
)
