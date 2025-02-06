;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/javascript/highlights.scm
;; Licensed under the Apache License 2.0
; inherits: ecma,jsx
; Parameters
(formal_parameters
  (identifier) @variable.parameter
)

(formal_parameters
  (rest_pattern
    (identifier) @variable.parameter
  )
)

; ({ a }) => null
(formal_parameters
  (object_pattern
    (shorthand_property_identifier_pattern) @variable.parameter
  )
)

; ({ a = b }) => null
(formal_parameters
  (object_pattern
    (object_assignment_pattern
      (shorthand_property_identifier_pattern) @variable.parameter
    )
  )
)

; ({ a: b }) => null
(formal_parameters
  (object_pattern
    (pair_pattern
      value: (identifier) @variable.parameter
    )
  )
)

; ([ a ]) => null
(formal_parameters
  (array_pattern
    (identifier) @variable.parameter
  )
)

; ({ a } = { a }) => null
(formal_parameters
  (assignment_pattern
    (object_pattern
      (shorthand_property_identifier_pattern) @variable.parameter
    )
  )
)

; ({ a = b } = { a }) => null
(formal_parameters
  (assignment_pattern
    (object_pattern
      (object_assignment_pattern
        (shorthand_property_identifier_pattern) @variable.parameter
      )
    )
  )
)

; a => null
(arrow_function
  parameter: (identifier) @variable.parameter
)

; optional parameters
(formal_parameters
  (assignment_pattern
    left: (identifier) @variable.parameter
  )
)

; punctuation
(optional_chain) @punctuation.delimiter
