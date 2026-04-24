;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/htmldjango/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (paired_comment)
    (unpaired_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  (content) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)
