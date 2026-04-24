;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/prisma/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (developer_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
