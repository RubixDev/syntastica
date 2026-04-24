;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vento/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (content) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)

(
  (code) @injection.content
  (#set! injection.language "javascript")
)
