;; Forked from https://raw.githubusercontent.com/ember-tooling/tree-sitter-glimmer-javascript/5cc865a2a0a77cbfaf5062c8fcf2a9919bd54f87/queries/glimmer_javascript/highlights.scm
; inherits: ecma
(glimmer_opening_tag) @tag.builtin

(glimmer_closing_tag) @tag.builtin

;
;
; Copied from the javascript queries
;
;
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
