;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/fsharp/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (line_comment)
    (block_comment_content)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  (line_comment) @injection.content
  (#lua-match? @injection.content "^///")
  (#offset! @injection.content 0 3 0 0)
  (#set! injection.language "xml")
  (#set! injection.combined)
)
