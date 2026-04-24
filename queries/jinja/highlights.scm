;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/jinja/highlights.scm
;; Licensed under the Apache License 2.0
; inherits: jinja_inline
[
  "{{"
  "{{-"
  "{{+"
  "+}}"
  "-}}"
  "}}"
  "{%"
  "{%-"
  "{%+"
  "+%}"
  "-%}"
  "%}"
] @keyword.directive

; TODO: only match raw
(raw_start) @keyword
