;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/embedded_template/injections.scm
;; Licensed under the Apache License 2.0
(
  (content) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)

(comment) @comment

(directive
  (code) @javascript @combined
)

(output_directive
  (code) @javascript @combined
)

(
  (code) @injection.content
  (#set! injection.language "javascript")
  (#set! injection.combined)
)
