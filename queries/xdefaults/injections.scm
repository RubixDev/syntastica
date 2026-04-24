;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/xresources/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (preprocessor_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
