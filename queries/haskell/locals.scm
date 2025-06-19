;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/haskell/locals.scm
;; Licensed under the Apache License 2.0
(signature
  name: (variable)
) @local.definition

(function
  name: (variable)
) @local.definition

(pattern/variable) @local.definition

(expression/variable) @local.reference
