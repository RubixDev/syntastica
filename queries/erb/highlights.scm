;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/embedded_template/highlights.scm
;; Licensed under the Apache License 2.0
(comment_directive) @comment

[
  "<%"
  "<%_"
  ; crates.io skip
  "<%|"
  "<%="
  ; crates.io skip
  "<%=="
  ; crates.io skip
  "<%|="
  ; crates.io skip
  "<%|=="
  "<%-"
  "<%#"
  "<%graphql"
  "%>"
  "-%>"
  "_%>"
  "=%>"
] @keyword
