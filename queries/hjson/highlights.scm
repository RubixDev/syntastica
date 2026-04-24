;; Forked from https://raw.githubusercontent.com/winston0410/tree-sitter-hjson/02fa3b79b3ff9a296066da6277adfc3f26cbc9e0/queries/highlights.scm
(pair
  key: (_) @keyword
)

(string) @string

(object
  "{" @escape
  (_)
  "}" @escape
)

(comment) @comment
