;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/robot/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (extra_text)
  ] @injection.content
  (#set! injection.language "comment")
)
