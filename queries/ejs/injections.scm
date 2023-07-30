;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/embedded_template/injections.scm
;; Licensed under the Apache License 2.0
(content) @html @combined

(comment) @comment

(directive
  (code) @javascript @combined
)

(output_directive
  (code) @javascript @combined
)

(graphql_directive
  (code) @graphql
)
