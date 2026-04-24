;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/squirrel/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (verbatim_string) @injection.content
  (#lua-match? @injection.content "^@\"<html")
  (#set! injection.language "html")
  (#offset! @injection.content 0 2 0 -1)
)

(
  (verbatim_string) @injection.content
  (#lua-match? @injection.content "@\"<!DOCTYPE html>")
  (#set! injection.language "html")
  (#offset! @injection.content 0 2 0 -1)
)
