;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/elsa/locals.scm
;; Licensed under the Apache License 2.0
[
  (source_file)
  (reduction)
] @local.scope

(identifier) @local.reference

(function) @local.definition.function

(method) @local.definition.method

(parameter) @local.definition.parameter
