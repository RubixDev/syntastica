;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/mlir/locals.scm
;; Licensed under the Apache License 2.0
(region) @local.scope

(func_arg_list
  (value_use) @local.definition.var
)

(block_arg_list
  (value_use) @local.definition.var
)

(op_result
  (value_use) @local.definition.var
)

(value_use) @local.reference
