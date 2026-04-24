;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ungrammar/locals.scm
;; Licensed under the Apache License 2.0
(grammar) @local.scope

[
  (definition)
  (label_name)
] @local.definition

(identifier) @local.reference
