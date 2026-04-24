;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/razor/injections.scm
;; Licensed under the Apache License 2.0
; inherits: c_sharp
(
  [
    (html_comment)
    (razor_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  (element) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)
