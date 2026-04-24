;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vimdoc/injections.scm
;; Licensed under the Apache License 2.0
(
  (codeblock
    (language) @injection.language
    (code) @injection.content
  )
  (#set! injection.include-children)
)
