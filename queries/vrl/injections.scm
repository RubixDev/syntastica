;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vrl/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (regex) @injection.content
  (#offset! @injection.content 0 2 0 -1)
  (#set! injection.language "regex")
)
