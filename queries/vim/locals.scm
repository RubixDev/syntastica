;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vim/locals.scm
;; Licensed under the Apache License 2.0
[
  (script_file)
  (function_definition)
] @local.scope

(function_declaration
  name: (identifier) @local.definition.function
)

(function_declaration
  parameters: (parameters
    (identifier) @local.definition.parameter
  )
)

(let_statement
  [
    (scoped_identifier)
    (identifier)
  ] @local.definition.var
)

(identifier) @local.reference
