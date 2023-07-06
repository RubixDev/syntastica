;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ql/locals.scm
;; Licensed under the Apache License 2.0
; SCOPES
(module) @scope

(dataclass) @scope

(datatype) @scope

;; TODO does not work
;(classMember (body)  @scope)
(memberPredicate
  (body) @scope
)

(classlessPredicate
  (body) @scope
)

(quantified
  (conjunction) @scope
)

(select) @scope

; DEFINITIONS
; module
(module
  name: (moduleName) @definition.namespace
)

; classes
(dataclass
  name: (className) @definition.type
)

(datatype
  name: (className) @definition.type
)

; predicates
(charpred
  (className) @definition.method
)

(memberPredicate
  name: (predicateName) @definition.method
)

(classlessPredicate
  name: (predicateName) @definition.function
)

; variables
(varDecl
  (varName
    (simpleId) @definition.var
  )
)

; REFERENCES
(simpleId) @reference
