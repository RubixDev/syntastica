;; Forked from https://github.com/ursalang/tree-sitter-ursa/blob/main/queries/locals.scm
;; Licensed under the ISC licence
; Scopes
;-------
[
  (block)
  (fn)
] @local.scope

; Definitions
;------------
(let
  identifier: (identifier) @local.definition
)

(for
  identifier: (identifier) @local.definition
)

; References
;------------
(identifier) @local.reference
