;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gitattributes/locals.scm
;; Licensed under the Apache License 2.0
(macro_def
  (attr_name) @local.definition.macro
)

(attribute
  (attr_name) @local.reference
)

(attribute
  (builtin_attr) @local.reference
)
