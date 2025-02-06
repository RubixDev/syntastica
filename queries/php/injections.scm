;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/php/injections.scm
;; Licensed under the Apache License 2.0
; inherits: php_only
(
  (text) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)
