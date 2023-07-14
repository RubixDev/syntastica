;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/markdown/injections.scm
;; Licensed under the Apache License 2.0
(fenced_code_block
  (info_string
    (language) @language
  )
  (code_fence_content) @content
  (#set! injection.include-unnamed-children)
)

(
  (html_block) @html @combined
)

(
  (minus_metadata) @yaml
  (#offset! @yaml 1 0 -1 0)
)

(
  (plus_metadata) @toml
  (#offset! @toml 1 0 -1 0)
)

(
  [
    (inline)
    (pipe_table_cell)
  ] @markdown_inline
  (#set! injection.include-unnamed-children)
)
