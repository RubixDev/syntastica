;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/latex/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (line_comment)
    (block_comment)
    (comment_environment)
  ] @injection.content
  (#set! injection.language "comment")
)

(pycode_environment
  code: (source_code) @injection.content
  (#set! injection.language "python")
)

(sagesilent_environment
  code: (source_code) @injection.content
  (#set! injection.language "python")
)

(sageblock_environment
  code: (source_code) @injection.content
  (#set! injection.language "python")
)

(luacode_environment
  code: (source_code) @injection.content
  (#set! injection.language "lua")
)

(asy_environment
  code: (source_code) @injection.content
  (#set! injection.language "c")
)

(asydef_environment
  code: (source_code) @injection.content
  (#set! injection.language "c")
)

(minted_environment
  (begin
    language: (curly_group_text
      (text) @injection.language
    )
  )
  (source_code) @injection.content
)
