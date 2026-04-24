;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/enforce/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (comment_block)
    (comment_line)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  [
    (doc_block)
    (doc_line)
  ] @injection.content
  (#set! injection.language "doxygen")
)

; TODO: string and print (numbered) format injection
