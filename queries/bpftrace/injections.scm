;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/bpftrace/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (c_struct)
    (c_preproc)
    (c_preproc_block)
  ] @injection.content
  (#set! injection.language "c")
)

(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
