(qldoc) @comment.documentation

[
  (line_comment)
  (block_comment)
] @comment @spell

(charpred
  (className) @function
)

(classlessPredicate
  name: (predicateName) @function
)

(memberPredicate
  name: (predicateName) @function
)

(aritylessPredicateExpr
  (literalId) @function
)

(string) @string

(float) @float

(integer) @number

(varName) @variable

(importModuleExpr
  qualName: (simpleId) @variable
)

(datatype
  name: (className) @type.definition
)

(typeExpr
  name: (className) @type
)

(dataclass
  name: (className) @type
)

(module
  name: (moduleName) @namespace
)

(moduleExpr
  (simpleId) @namespace
)

[
  ","
  "|"
] @punctuation.delimiter

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

[
  "<"
  "<="
  "="
  ">"
  ">="
  "-"
  "!="
  "/"
  "*"
  "%"
  "+"
  "::"
] @operator

(annotName) @attribute

[
  "boolean"
  "float"
  "int"
  "date"
  "string"
] @type.builtin

[
  (this)
  (super)
] @variable.builtin

[
  (true)
  (false)
] @boolean

[
  "asc"
  "desc"
] @type.qualifier

[
  "forall"
  "forex"
] @repeat

[
  "if"
  "then"
  "else"
] @conditional

("import"
  @include
)

[
  "avg"
  "any"
  "count"
  "concat"
  "exists"
  "max"
  "min"
  "instanceof"
  "rank"
  "sum"
  "strictconcat"
  "strictcount"
  "strictsum"
] @function.builtin

[
  "and"
  "not"
  "or"
] @keyword.operator

[
  "as"
  "by"
  "class"
  "extends"
  "from"
  "implies"
  "in"
  "module"
  "newtype"
  "order"
  "select"
  "where"
  (predicate)
  (result)
  (specialId)
] @keyword
