;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/rnoweb/injections.scm
;; Licensed under the Apache License 2.0
(
  (latex) @injection.content
  (#set! injection.language "latex")
  (#set! injection.combined)
)

(rchunk
  (renv_content) @injection.content
  (#set! injection.language "r")
  (#set! injection.combined)
)

(rinline
  (renv_content) @injection.content
  (#set! injection.language "r")
)
