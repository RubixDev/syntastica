;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/markdown/injections.scm
;; Licensed under the Apache License 2.0
(fenced_code_block
  (info_string
    (language) @injection.language
  )
  (code_fence_content) @injection.content
  (#set! injection.include-unnamed-children)
)

(
  (html_block) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
  (#set! injection.include-children)
)

(
  (minus_metadata) @injection.content
  (#set! injection.language "yaml")
  (#offset! @injection.content 1 0 -1 0)
  (#set! injection.include-children)
)

(
  (plus_metadata) @injection.content
  (#set! injection.language "toml")
  (#offset! @injection.content 1 0 -1 0)
  (#set! injection.include-children)
)

(
  [
    (inline)
    (pipe_table_cell)
  ] @injection.content
  (#set! injection.include-unnamed-children)
  (#set! injection.language "markdown_inline")
)
