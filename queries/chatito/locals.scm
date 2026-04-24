;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/chatito/locals.scm
;; Licensed under the Apache License 2.0
; Definitions
(intent_def
  (intent) @local.definition
)

(slot_def
  (slot) @local.definition
)

(alias_def
  (alias) @local.definition
)

; References
(slot_ref
  (slot) @local.reference
)

(alias_ref
  (alias) @local.reference
)
