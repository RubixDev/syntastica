;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/properties/locals.scm
;; Licensed under the Apache License 2.0
(property
  (key) @local.definition
)

(substitution
  (key) @local.reference
)
