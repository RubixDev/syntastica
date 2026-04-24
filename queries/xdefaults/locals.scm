;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/xresources/locals.scm
;; Licensed under the Apache License 2.0
(define_directive
  name: (identifier) @local.definition.macro
)

(define_function_directive
  name: (identifier) @local.definition.macro
)

(parameters
  (identifier) @local.definition.parameter
)

(identifier) @local.reference

(resources) @local.scope
