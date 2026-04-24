;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/tablegen/injections.scm
;; Licensed under the Apache License 2.0
(
  (code) @injection.content
  (#set! injection.language "cpp")
  (#offset! @injection.content 0 2 0 -2)
)

(
  (tablegen_file
    (comment) @injection.content
  )
  (#lua-match? @injection.content "^.*RUN")
  (#set! injection.language "bash")
  (#offset! @injection.content 0 8 0 0)
)
