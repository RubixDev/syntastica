;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ocamllex/injections.scm
;; Licensed under the Apache License 2.0
(
  (ocaml) @injection.content
  (#set! injection.language "ocaml")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
