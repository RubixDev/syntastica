;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/foam/locals.scm
;; Licensed under the Apache License 2.0
(dict) @local.scope

(dict
  key: (_) @local.definition.type
)

(key_value
  keyword: (_) @local.definition.parameter
)

(key_value
  value: (macro
    (identifier)*
  )* @local.reference
)
