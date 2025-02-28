;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/lalrpop/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  [
    (normal_action)
    (failible_action)
  ] @injection.content
  (#set! injection.language "rust")
)

(
  (use) @injection.content
  (#set! injection.language "rust")
)

(
  (regex_literal) @injection.content
  (#set! injection.language "regex")
  (#offset! @injection.content 0 2 0 -1)
)
