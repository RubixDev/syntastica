;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/purescript/locals.scm
;; Licensed under the Apache License 2.0
(signature
  name: (variable)
) @local.definition.type

(function
  name: (variable)
) @local.definition.function

(pat_name
  (variable)
) @local.definition

(exp_name
  (variable)
) @local.reference
