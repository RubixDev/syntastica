[
  (line_comment)
  (block_comment)
] @comment @spell

(
  (string_literal) @string.documentation
  .
  [
    (module_definition)
    (abstract_definition)
    (struct_definition)
    (function_definition)
    (assignment)
    (const_declaration)
  ]
)

(prefixed_command_literal
  prefix: (identifier) @function.macro
) @string.special

(command_literal) @string.special

(prefixed_string_literal
  prefix: (identifier) @function.macro
) @string

(string_literal) @string

(escape_sequence) @string.escape

(character_literal) @character

(
  (identifier) @constant.builtin
  (#match? @constant.builtin "^(nothing|missing)$")
)

(
  (identifier) @float
  (#match? @float "^(NaN|NaN16|NaN32|Inf|Inf16|Inf32)$")
)

(float_literal) @float

(integer_literal) @number

(boolean_literal) @boolean

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

["..."] @punctuation.special

[
  ","
  "."
  ";"
  "::"
  "->"
] @punctuation.delimiter

(where_expression
  "where" @keyword.operator
)

(where_clause
  "where" @keyword.operator
)

(for_binding
  "in" @keyword.operator
)

(
  (operator) @keyword.operator
  (#match? @keyword.operator "^(in|isa)$")
)

(range_expression
  ":" @operator
)

(adjoint_expression
  "'" @operator
)

[
  "="
  "∈"
  (operator)
] @operator

[
  "const"
  "mutable"
] @type.qualifier

(return_statement
  "return" @keyword.return
)

(do_clause
  [
    "do"
    "end"
  ] @keyword.function
)

(function_definition
  [
    "function"
    "end"
  ] @keyword.function
)

(macro_definition
  [
    "macro"
    "end" @keyword
  ]
)

(export_statement
  "export" @include
)

(import_alias
  "as" @include
)

(import_statement
  [
    "import"
    "using"
  ] @include
)

(module_definition
  [
    "module"
    "baremodule"
    "end"
  ] @include
)

[
  (break_statement)
  (continue_statement)
] @repeat

(for_clause
  "for" @repeat
)

(while_statement
  [
    "while"
    "end"
  ] @repeat
)

(for_statement
  [
    "for"
    "end"
  ] @repeat
)

(catch_clause
  "catch" @exception
)

(finally_clause
  "finally" @exception
)

(try_statement
  [
    "try"
    "end"
  ] @exception
)

(ternary_expression
  [
    "?"
    ":"
  ] @conditional.ternary
)

(if_clause
  "if" @conditional
)

(else_clause
  "else" @conditional
)

(elseif_clause
  "elseif" @conditional
)

(if_statement
  [
    "if"
    "end"
  ] @conditional
)

(let_statement
  [
    "let"
    "end"
  ] @keyword
)

(quote_statement
  [
    "quote"
    "end"
  ] @keyword
)

(compound_statement
  [
    "begin"
    "end"
  ] @keyword
)

[
  "global"
  "local"
  "macro"
  "struct"
  "end"
] @keyword

(
  (identifier) @variable.builtin
  (#match? @variable.builtin "^(begin|end)$")
  (#has-ancestor? @variable.builtin range_expression)
)

(
  (identifier) @variable.builtin
  (#match? @variable.builtin "^(begin|end)$")
  (#has-ancestor? @variable.builtin index_expression)
)

(
  (identifier) @type.builtin
  (#match? @type.builtin "^(Type|DataType|Any|Union|UnionAll|Tuple|NTuple|NamedTuple|Val|Nothing|Some|Enum|Expr|Symbol|Module|Function|ComposedFunction|Number|Real|AbstractFloat|Integer|Signed|AbstractIrrational|Fix1|Fix2|Missing|Cmd|EnvDict|VersionNumber|ArgumentError|AssertionError|BoundsError|CompositeException|DimensionMismatch|DivideError|DomainError|EOFError|ErrorException|InexactError|InterruptException|KeyError|LoadError|MethodError|OutOfMemoryError|ReadOnlyMemoryError|OverflowError|ProcessFailedException|StackOverflowError|SystemError|TypeError|UndefKeywordError|UndefRefError|UndefVarError|StringIndexError|InitError|ExponentialBackOff|Timer|AsyncCondition|ParseError|QuoteNode|IteratorSize|IteratorEltype|AbstractRange|OrdinalRange|AbstractUnitRange|StepRange|UnitRange|LinRange|AbstractDict|Dict|IdDict|WeakKeyDict|ImmutableDict|AbstractSet|Set|BitSet|Pair|Pairs|OneTo| StepRangeLen|RoundingMode|Float16|Float32|Float64|BigFloat|Bool|Int|Int8|UInt8|Int16|UInt16|Int32|UInt32|Int64|UInt64|Int128|UInt128|BigInt|Complex|Rational|Irrational|AbstractChar|Char|SubString|Regex|SubstitutionString|RegexMatch|AbstractArray|AbstractVector|AbstractMatrix|AbstractVecOrMat|Array|UndefInitializer|Vector|Matrix|VecOrMat|DenseArray|DenseVector|DenseMatrix|DenseVecOrMat|StridedArray|StridedVector|StridedMatrix|StridedVecOrMat|BitArray|Dims|SubArray|Task|Condition|Event|Semaphore|AbstractLniock|ReentrantLock|Channel|Atomic|SpinLock|RawFD|IOStream|IOBuffer|AbstractDisplay|MIME|TextDisplay|PartialQuickSort|Ordering|ReverseOrdering|By|Lt|Perm|Stateful|CFunction|Ptr|Ref|Cchar|Cuchar|Cshort|Cstring|Cushort|Cint|Cuint|Clong|Culong|Clonglong|Culonglong|Cintmax_t|Cuintmax_t|Csize_t|Cssize_t|Cptrdiff_t|Cwchar_t|Cwstring|Cfloat|Cdouble|Tmstruct|StackFrame|StackTrace)$")
)

(where_clause
  (curly_expression
    (_) @type
  )
)

(where_clause
  (identifier) @type
)

(short_function_definition
  return_type: (identifier) @type
)

(function_definition
  return_type: (identifier) @type
)

(typed_expression
  (identifier) @type
  .
)

(type_parameter_list
  (identifier) @type
)

(parametrized_type_expression
  (_) @type
  (curly_expression
    (_) @type
  )
)

(type_clause
  [
    "<:"
    ">:"
  ] @operator
  [
    (identifier) @type
    (field_expression
      (identifier) @type
      .
    )
  ]
)

(struct_definition
  name: (identifier) @type
)

(primitive_definition
  name: (identifier) @type.definition
) @keyword

(abstract_definition
  name: (identifier) @type.definition
) @keyword

(function_expression
  .
  (identifier) @parameter
)

(typed_parameter
  parameter: (identifier)
  ? @parameter
  type: (_) @type
)

(slurp_parameter
  (identifier) @parameter
)

(optional_parameter
  .
  (identifier) @parameter
)

(parameter_list
  (identifier) @parameter
)

(
  (identifier) @function.builtin
  (#match? @function.builtin "^(_abstracttype|_apply_iterate|_apply_pure|_call_in_world|_call_in_world_total|_call_latest|_equiv_typedef|_expr|_primitivetype|_setsuper!|_structtype|_typebody!|_typevar|applicable|apply_type|arrayref|arrayset|arraysize|const_arrayref|donotdelete|fieldtype|get_binding_type|getfield|ifelse|invoke|isa|isdefined|modifyfield!|nfields|replacefield!|set_binding_type!|setfield!|sizeof|svec|swapfield!|throw|tuple|typeassert|typeof)$")
)

(broadcast_call_expression
  (field_expression
    (identifier) @function.call
    .
  )
)

(broadcast_call_expression
  (identifier) @function.call
)

(call_expression
  (field_expression
    (identifier) @function.call
    .
  )
)

(call_expression
  (identifier) @function.call
)

(short_function_definition
  name: (field_expression
    (identifier) @function
    .
  )
)

(function_definition
  name: (field_expression
    (identifier) @function
    .
  )
)

(short_function_definition
  name: (identifier) @function
)

(function_definition
  name: (identifier) @function
)

(field_expression
  (identifier) @field
  .
)

(quote_expression
  ":"
  [
    (identifier)
    (operator)
  ]
) @symbol

(macro_definition
  name: (identifier) @function.macro
)

(macro_identifier
  (identifier) @function.macro
)

(macro_identifier) @function.macro

(identifier) @variable
