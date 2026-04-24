;; Forked from https://raw.githubusercontent.com/virchau13/tree-sitter-astro/213f6e6973d9b456c6e50e86f19f66877e7ef0ee/queries/highlights.scm
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
] @punctuation.bracket
