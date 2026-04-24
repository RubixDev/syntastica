;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/apex/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  (block_comment) @injection.content
  (#lua-match? @injection.content "/[*][*][%s]")
  (#set! injection.language "javadoc")
)

; markdown-style javadocs https://openjdk.org/jeps/467
(
  (line_comment) @injection.content
  (#lua-match? @injection.content "^///%s")
  (#set! injection.language "javadoc")
)
