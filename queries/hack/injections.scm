;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hack/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (heredoc)
  ] @injection.content
  (#set! injection.language "comment")
)
