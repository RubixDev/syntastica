;; Forked from https://github.com/helix-editor/helix/blob/master/runtime/queries/rust/locals.scm
;; Licensed under the Mozilla Public License 2.0
; Scopes
[
  (function_item)
  (closure_expression)
  (block)
] @local.scope

; Definitions
(parameter
  (identifier) @local.definition
)

(closure_parameters
  (identifier) @local.definition
)

; References
(identifier) @local.reference
