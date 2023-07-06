;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ruby/injections.scm
;; Licensed under the Apache License 2.0
(comment) @comment

(heredoc_body
  (heredoc_content) @content
  (heredoc_end) @language
)

(regex
  (string_content) @regex
)
