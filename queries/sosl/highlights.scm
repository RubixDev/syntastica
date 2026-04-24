;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/sosl/highlights.scm
;; Licensed under the Apache License 2.0
(find_clause
  (term) @string
)

(sobject_return
  (identifier) @type
)

(with_type
  (_
    "=" @operator
  )
)

[
  "ALL"
  "DIVISION"
  "EMAIL"
  "FIND"
  "ListView"
  "HIGHLIGHT"
  "IN"
  "METADATA"
  "NAME"
  "NETWORK"
  "PHONE"
  "PricebookId"
  "RETURNING"
  "SIDEBAR"
  "SNIPPET"
  "SPELL_CORRECTION"
  "target_length"
  "USING"
] @keyword
