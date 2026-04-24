;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/puppet/injections.scm
;; Licensed under the Apache License 2.0
(
  (regex) @injection.content
  (#set! injection.language "regex")
  (#offset! @injection.content 0 1 0 -1)
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
