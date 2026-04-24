;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/sourcepawn/injections.scm
;; Licensed under the Apache License 2.0
; Parse JSDoc annotations in comments
(
  (comment) @injection.content
  (#set! injection.language "jsdoc")
)
