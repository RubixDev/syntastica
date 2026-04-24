;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/tera/injections.scm
;; Licensed under the Apache License 2.0
(frontmatter
  (content) @injection.content
  (#set! injection.language "yaml")
  (#set! injection.combined)
)

(
  (comment_tag) @injection.content
  (#set! injection.language "comment")
)
