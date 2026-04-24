;; Forked from https://gitlab.com/jirgn/tree-sitter-fusion/-/raw/19db2f47ba4c3a0f6238d4ae0e2abfca16e61dd6/queries/locals.scm
;; Fusion base
(block) @scope

(namespace_declaration
  (alias_namespace) @definition.namespace
)

(property
  (path
    (path_part) @definition.field
  )
)

(type
  namespace: (package_name)? @definition.namespace
  name: (type_name) @definition.type
)

;; Eel Expressions
(eel_arrow_function) @scope

(eel_object) @scope

(eel_parameter) @definition.parameter

(eel_object_pair
  key: (eel_property_name) @definition.field
)
