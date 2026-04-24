;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/fusion/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (afx_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
