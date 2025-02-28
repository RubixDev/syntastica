;; Forked from https://github.com/rush-rs/tree-sitter-rush/blob/main/queries/rush/locals.scm
;; Licensed under the MIT license
(block) @local.scope

; Functions
(function_definition
  name: (ident) @local.definition.function
)

(parameter
  name: (ident) @local.definition.parameter
)

; Variables
(let_stmt
  name: (ident) @local.definition.var
)

(for_stmt
  name: (ident) @local.definition.var
)

; References
(ident) @local.reference
