;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/bicep/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (diagnostic_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
