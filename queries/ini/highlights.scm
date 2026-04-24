;; Forked from https://raw.githubusercontent.com/justinmk/tree-sitter-ini/e4018b5176132b4f3c5d6e61cea383f42288d0f5/queries/highlights.scm
(section_name
  (text) @type
)

; consistency with toml
(comment) @comment @spell

[
  "["
  "]"
] @punctuation.bracket

"=" @operator

(setting
  (setting_name) @property
)

; (setting_value) @none ; grammar does not support subtypes
