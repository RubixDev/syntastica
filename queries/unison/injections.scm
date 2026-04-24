;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/unison/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (doc_block) @injection.content
  (#set! injection.language "markdown")
)
