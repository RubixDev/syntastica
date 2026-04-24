;; Forked from https://raw.githubusercontent.com/tree-sitter/tree-sitter-html/73a3947324f6efddf9e17c0ea58d454843590cc0/queries/highlights.scm
(tag_name) @tag

(erroneous_end_tag_name) @tag.error

(doctype) @constant

(attribute_name) @attribute

(attribute_value) @string

(comment) @comment

[
  "<"
  ">"
  "</"
  "/>"
] @punctuation.bracket
