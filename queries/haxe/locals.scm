;; Forked from https://raw.githubusercontent.com/vantreeseba/tree-sitter-haxe/a55f3e2cf1e4449200fd089a80d3af642bcf5f94/queries/locals.scm
; Scopes
[
  (block)
  (function_declaration)
] @scope @local.scope

; Definitions
(function_arg
  name: (identifier) @definition.parameter
)

(variable_declaration
  name: (identifier) @local.definition
)

; References
(block
  (identifier)
) @local.reference
