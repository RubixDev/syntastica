;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hcl/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(heredoc_template
  (template_literal) @injection.content
  (heredoc_identifier) @injection.language
)
