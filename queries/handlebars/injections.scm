;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/glimmer/injections.scm
;; Licensed under the Apache License 2.0
; comments
(
  (comment_statement) @injection.content
  (#set! injection.language "comment")
)

; <style> tags
(
  (style_element
    (raw_text) @injection.content
  )
  (#set! injection.language "css")
)

; <script> tags
(
  (script_element
    (raw_text) @injection.content
  )
  (#set! injection.language "javascript")
  (#set! injection.include-children)
)
