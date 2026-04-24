;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/kconfig/locals.scm
;; Licensed under the Apache License 2.0
[
  (symbol)
  (string)
] @local.reference

[
  (config)
  (menuconfig)
  (choice)
  (comment_entry)
  (menu)
  (if)
] @local.scope

(type_definition
  (string) @local.definition.var
)

(type_definition
  (input_prompt
    (string) @local.definition.var
  )
)

(type_definition_default
  (expression
    (string) @local.definition.var
  )
)
