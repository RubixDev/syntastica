;; Forked from https://github.com/rush-rs/tree-sitter-rush/blob/main/queries/rush/locals.scm
;; Licensed under the MIT license
(block) @scope

; Functions
(function_definition
  name: (ident) @definition.function
)

(parameter
  name: (ident) @definition.parameter
)

; Variables
(let_stmt
  name: (ident) @definition.var
)

(for_stmt
  name: (ident) @definition.var
)

; References
(ident) @reference
