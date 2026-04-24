;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/superhtml/injections.scm
;; Licensed under the Apache License 2.0
(
  (script_element
    (raw_text) @injection.content
  )
  (#set! injection.language "javascript")
)

(
  (style_element
    (raw_text) @injection.content
  )
  (#set! injection.language "css")
)
