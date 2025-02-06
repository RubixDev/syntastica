;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/typescript/locals.scm
;; Licensed under the Apache License 2.0
; inherits: ecma
(required_parameter
  (identifier) @local.definition
)

(optional_parameter
  (identifier) @local.definition
)

; x => x
(arrow_function
  parameter: (identifier) @local.definition.parameter
)

; ({ a }) => null
(required_parameter
  (object_pattern
    (shorthand_property_identifier_pattern) @local.definition.parameter
  )
)

; ({ a: b }) => null
(required_parameter
  (object_pattern
    (pair_pattern
      value: (identifier) @local.definition.parameter
    )
  )
)

; ([ a ]) => null
(required_parameter
  (array_pattern
    (identifier) @local.definition.parameter
  )
)

(required_parameter
  (rest_pattern
    (identifier) @local.definition.parameter
  )
)
