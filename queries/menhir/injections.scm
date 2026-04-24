;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/menhir/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment)
    (line_comment)
    (ocaml_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  (ocaml) @injection.content
  (#set! injection.language "ocaml")
)
