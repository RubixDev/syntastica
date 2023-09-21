(quoter) @function.call

(
  (constructor) @boolean
  (#match? @boolean "^(True|False)$")
)

(constructor) @constructor

(type_variable) @type

(type_star) @type

(type) @type

(exp_apply
  .
  (exp_name
    (qualified_variable
      (variable) @function.call
    )
  )
)

(exp_apply
  .
  (exp_name
    (variable) @function.call
  )
)

(exp_infix
  (exp_name) @function.call
  (#set! "priority" 101)
)

(exp_section_left
  (variable) @operator
)

(exp_section_right
  (variable) @operator
)

(exp_infix
  (variable) @operator
)

(
  (signature
    (variable) @_type
    (forall
      (context
        (fun)
      )
    )
  )
  .
  (function
    (variable) @function
  )
  (#eq? @function @_type)
)

(
  (signature
    (variable) @function
    (forall
      (context
        (fun)
      )
    )
  )
  .
  (function
    (variable)
  )
)

(
  (signature
    (variable) @_type
    (context
      (fun)
    )
  )
  .
  (function
    (variable) @function
  )
  (#eq? @function @_type)
)

(
  (signature
    (variable) @function
    (context
      (fun)
    )
  )
  .
  (function
    (variable)
  )
)

(
  (signature
    (variable) @_type
    (fun)
  )
  .
  (function
    (variable) @function
  )
  (#eq? @function @_type)
)

(
  (signature
    (variable) @function
    (fun)
  )
  .
  (function
    (variable)
  )
)

(function
  name: (variable) @function
  rhs: (exp_lambda)
)

(function
  name: (variable) @function
  patterns: (patterns)
)

(signature
  name: (variable) @variable
)

(pat_wildcard) @variable

(variable) @variable

[
  (where)
  "let"
  "in"
  "class"
  "instance"
  "pattern"
  "data"
  "newtype"
  "family"
  "type"
  "as"
  "hiding"
  "deriving"
  "via"
  "stock"
  "anyclass"
  "do"
  "mdo"
  "rec"
  "infix"
  "infixl"
  "infixr"
] @keyword

(module) @namespace

[
  (operator)
  (constructor_operator)
  (type_operator)
  (tycon_arrow)
  (qualified_module)
  (qualified_type)
  (qualified_variable)
  (all_names)
  (wildcard)
  "."
  ".."
  "="
  "|"
  "::"
  "=>"
  "->"
  "<-"
  "\\"
  "`"
  "@"
] @operator

[
  "import"
  "qualified"
  "module"
] @include

[
  "if"
  "then"
  "else"
  "case"
  "of"
] @conditional

(pragma) @preproc

[
  "forall"
  "∀"
] @repeat

[
  (comma)
  ";"
] @punctuation.delimiter

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

(comment) @comment

(con_unit) @symbol

(string) @string

(char) @character

(exp_literal
  (float)
) @float

(exp_negation) @number

(integer) @number
