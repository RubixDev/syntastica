;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/dot/injections.scm
;; Licensed under the Apache License 2.0
(
  (html_internal) @injection.content
  (#set! injection.language "html")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
