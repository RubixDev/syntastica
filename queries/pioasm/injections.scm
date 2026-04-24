;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/pioasm/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  (code_block
    (code_block_language) @_language
    (code_block_body) @injection.content
  )
  (#eq? @_language "c-sdk")
  (#set! injection.language "c")
)

(code_block
  (code_block_language) @injection.language
  (code_block_body) @injection.content
)
