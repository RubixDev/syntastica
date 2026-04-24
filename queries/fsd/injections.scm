;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/facility/injections.scm
;; Licensed under the Apache License 2.0
(
  (remarks) @injection.content
  (#set! injection.language "markdown")
)

(
  [
    (comment)
    (doc_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
