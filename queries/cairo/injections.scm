;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/cairo/injections.scm
;; Licensed under the Apache License 2.0
(
  (python_code) @injection.content
  (#set! injection.language "python")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
