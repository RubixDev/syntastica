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
)

(
  (minus_metadata) @injection.content
  (#set! injection.language "yaml")
  (#offset! @injection.content 1 0 -1 0)
)

(
  (plus_metadata) @injection.content
  (#set! injection.language "toml")
  (#offset! @injection.content 1 0 -1 0)
)

(
  [
    (inline)
    (pipe_table_cell)
  ] @injection.content
  (#set! injection.language "markdown_inline")
  (#set! injection.include-unnamed-children)
)
