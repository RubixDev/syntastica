;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/wxml/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (raw_text) @injection.content
  (#set! injection.language "javascript")
  (#set! injection.include-children)
)

(
  (expression) @injection.content
  (#set! injection.language "javascript")
  (#set! injection.include-children)
)
