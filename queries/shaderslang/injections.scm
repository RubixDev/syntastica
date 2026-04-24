;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/slang/injections.scm
;; Licensed under the Apache License 2.0
(
  (preproc_arg) @injection.content
  (#set! injection.language "slang")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
