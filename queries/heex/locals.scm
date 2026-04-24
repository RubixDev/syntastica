;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/heex/locals.scm
;; Licensed under the Apache License 2.0
; HEEx tags, components, and slots are references
[
  (component_name)
  (slot_name)
  (tag_name)
] @local.reference

; Create a new scope within each HEEx tag, component, and slot
[
  (component)
  (slot)
  (tag)
] @local.scope
