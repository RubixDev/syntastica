;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/pkl/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (lineComment)
    (blockComment)
    (docComment)
  ] @injection.content
  (#set! injection.language "comment")
)
