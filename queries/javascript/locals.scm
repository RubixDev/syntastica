; inherits: ecma

; Both properties are matched here.
;
;   class Foo {
;     this.#bar = "baz";
;     this.quuz = "qux";
;   }
(field_definition
  property: [(property_identifier) (private_property_identifier)] @local.definition)

; this.foo = "bar"
(assignment_expression
  left: (member_expression
    object: (this)
    property: (property_identifier) @local.definition))

(formal_parameters
  (identifier) @local.definition)

; function(arg = []) {
(formal_parameters
  (assignment_pattern
    left: (identifier) @local.definition))

; x => x
(arrow_function
  parameter: (identifier) @local.definition)

;; ({ a }) => null
(formal_parameters
  (object_pattern
    (shorthand_property_identifier_pattern) @local.definition))

;; ({ a: b }) => null
(formal_parameters
  (object_pattern
    (pair_pattern
      value: (identifier) @local.definition)))

;; ([ a ]) => null
(formal_parameters
  (array_pattern
    (identifier) @local.definition))

(formal_parameters
  (rest_pattern
    (identifier) @local.definition))

; Both methods are matched here.
;
;   class Foo {
;     #bar(x) { x }
;     baz(y) { y }
;   }
(method_definition
  ([(property_identifier) (private_property_identifier)] @local.definition)
   (#set! definition.var.scope parent))

; this.foo()
(member_expression
  object: (this)
  property: (property_identifier) @local.reference)
