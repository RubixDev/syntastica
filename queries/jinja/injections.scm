;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/jinja/injections.scm
;; Licensed under the Apache License 2.0
(
  (inline) @injection.content
  (#set! injection.language "jinja_inline")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
