(module) @local.scope

(dataclass) @local.scope

(datatype) @local.scope

(memberPredicate
  (body) @local.scope
)

(classlessPredicate
  (body) @local.scope
)

(quantified
  (conjunction) @local.scope
)

(select) @local.scope

(module
  name: (moduleName) @local.definition
)

(dataclass
  name: (className) @local.definition
)

(datatype
  name: (className) @local.definition
)

(charpred
  (className) @local.definition
)

(memberPredicate
  name: (predicateName) @local.definition
)

(classlessPredicate
  name: (predicateName) @local.definition
)

(varDecl
  (varName
    (simpleId) @local.definition
  )
)

(simpleId) @local.reference
