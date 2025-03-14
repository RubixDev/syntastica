;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/markdown_inline/injections.scm
;; Licensed under the Apache License 2.0
(
  (html_tag) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)

(
  (latex_block) @injection.content
  (#set! injection.language "latex")
  (#set! injection.include-children)
)
