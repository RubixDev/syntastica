;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/smithy/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (documentation_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
