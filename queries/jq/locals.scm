;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/jq/locals.scm
;; Licensed under the Apache License 2.0
(funcdef
  (identifier) @local.definition.function
)

(funcdefargs
  (identifier) @local.definition.parameter
)

(funcname) @local.reference

(index
  (identifier) @local.reference
)
