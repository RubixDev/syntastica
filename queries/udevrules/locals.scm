;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/udev/locals.scm
;; Licensed under the Apache License 2.0
; labels
(assignment
  key:
  "LABEL"
  (value
    (content) @local.definition
  )
)

(assignment
  key:
  "GOTO"
  (value
    (content) @local.reference
  )
)

; env vars
(assignment
  key:
  "ENV"
  (env_var) @local.definition.var
)

(match
  key:
  "ENV"
  (env_var) @local.reference
)

(var_sub
  (env_var) @local.reference
)

; misc
[
  (attribute)
  (kernel_param)
  (seclabel)
] @local.reference
