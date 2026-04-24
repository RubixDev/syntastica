;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gaptst/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (gap_expression) @injection.content
  (#set! injection.language "gap")
)

(
  (input_line) @injection.content
  (#set! injection.language "gap")
  (#set! injection.combined)
)
