;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/rasi/locals.scm
;; Licensed under the Apache License 2.0
(rule_set
  (selectors
    (id_selector)
  )
) @local.scope

(block
  (declaration
    (property_name) @local.definition.var
  )
)

(reference_value
  name: (identifier) @local.reference
)
