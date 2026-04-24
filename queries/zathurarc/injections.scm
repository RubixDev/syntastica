;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/zathurarc/injections.scm
;; Licensed under the Apache License 2.0
(set_directive
  (option) @_option
  (string) @injection.content
  (#eq? @_option "synctex-editor-command")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "bash")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
