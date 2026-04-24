;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/earthfile/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (line_continuation_comment) @injection.content
  (#set! injection.language "comment")
)

(
  (shell_fragment) @injection.content
  (#set! injection.language "bash")
  (#set! injection.include-children)
)
