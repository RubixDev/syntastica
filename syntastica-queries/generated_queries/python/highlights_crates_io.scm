(ERROR) @error

(
  (identifier) @type.builtin
  (#match? @type.builtin "^(BaseException|Exception|ArithmeticError|BufferError|LookupError|AssertionError|AttributeError|EOFError|FloatingPointError|GeneratorExit|ImportError|ModuleNotFoundError|IndexError|KeyError|KeyboardInterrupt|MemoryError|NameError|NotImplementedError|OSError|OverflowError|RecursionError|ReferenceError|RuntimeError|StopIteration|StopAsyncIteration|SyntaxError|IndentationError|TabError|SystemError|SystemExit|TypeError|UnboundLocalError|UnicodeError|UnicodeEncodeError|UnicodeDecodeError|UnicodeTranslateError|ValueError|ZeroDivisionError|EnvironmentError|IOError|WindowsError|BlockingIOError|ChildProcessError|ConnectionError|BrokenPipeError|ConnectionAbortedError|ConnectionRefusedError|ConnectionResetError|FileExistsError|FileNotFoundError|InterruptedError|IsADirectoryError|NotADirectoryError|PermissionError|ProcessLookupError|TimeoutError|Warning|UserWarning|DeprecationWarning|PendingDeprecationWarning|SyntaxWarning|RuntimeWarning|FutureWarning|ImportWarning|UnicodeWarning|BytesWarning|ResourceWarning|bool|int|float|complex|list|tuple|range|str|bytes|bytearray|memoryview|set|frozenset|dict|type|object)$")
)

(
  (class_definition
    (block
      (function_definition
        name: (identifier) @constructor
      )
    )
  )
  (#match? @constructor "^(__new__|__init__)$")
)

(
  (class_definition
    body: (block
      (expression_statement
        (assignment
          left: (_
            (identifier) @field
          )
        )
      )
    )
  )
  (#match? @field "^[a-z][\\s\\S]*$")
)

(
  (class_definition
    body: (block
      (expression_statement
        (assignment
          left: (identifier) @field
        )
      )
    )
  )
  (#match? @field "^[a-z][\\s\\S]*$")
)

(class_definition
  superclasses: (argument_list
    (identifier) @type
  )
)

(class_definition
  body: (block
    (function_definition
      name: (identifier) @method
    )
  )
)

(class_definition
  name: (identifier) @type
)

[
  ","
  "."
  ":"
  ";"
  (ellipsis)
] @punctuation.delimiter

(type_conversion) @function.macro

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special
)

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(try_statement
  (else_clause
    "else" @exception
  )
)

(raise_statement
  "from" @exception
)

[
  "try"
  "raise"
  "finally"
] @exception

[
  "for"
  "while"
  "break"
  "continue"
] @repeat

[
  "if"
  "elif"
  "else"
  "match"
  "case"
] @conditional

(aliased_import
  "as" @include
)

"import" @include

(import_from_statement
  "from" @include
)

(future_import_statement
  "from" @include
  "__future__" @constant.builtin
)

(yield
  "from" @keyword.return
)

[
  "return"
  "yield"
] @keyword.return

[
  "async"
  "await"
] @keyword.coroutine

[
  "assert"
  "class"
  "exec"
  "global"
  "nonlocal"
  "pass"
  "print"
  "with"
  "as"
] @keyword

[
  "def"
  "lambda"
] @keyword.function

[
  "and"
  "in"
  "is"
  "not"
  "or"
  "del"
] @keyword.operator

[
  "-"
  "-="
  ":="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "@"
  "@="
  "|"
  "|="
  "~"
  "->"
] @operator

(function_definition
  body: (block
    .
    (expression_statement
      (string) @string.documentation @spell
    )
  )
)

(class_definition
  body: (block
    .
    (expression_statement
      (string) @string.documentation @spell
    )
  )
)

(module
  .
  (expression_statement
    (string) @string.documentation @spell
  )
)

(escape_sequence) @string.escape

(string) @string

(
  (module
    .
    (comment) @preproc
  )
  (#match? @preproc "^#!\\/")
)

(comment) @comment @spell

(float) @float

(integer) @number

(
  (identifier) @variable.builtin
  (#eq? @variable.builtin "cls")
)

(
  (identifier) @variable.builtin
  (#eq? @variable.builtin "self")
)

[
  (true)
  (false)
] @boolean

(none) @constant.builtin

(parameters
  (dictionary_splat_pattern
    (identifier) @parameter
  )
)

(parameters
  (list_splat_pattern
    (identifier) @parameter
  )
)

(typed_default_parameter
  (identifier) @parameter
)

(typed_parameter
  (identifier) @parameter
)

(default_parameter
  name: (identifier) @parameter
)

(keyword_argument
  name: (identifier) @parameter
)

(lambda_parameters
  (tuple_pattern
    (identifier) @parameter
  )
)

(lambda_parameters
  (identifier) @parameter
)

(parameters
  (identifier) @parameter
)

(
  (call
    function: (identifier) @_isinstance
    arguments: (argument_list
      (_)
      (identifier) @type
    )
  )
  (#eq? @_isinstance "isinstance")
)

(type
  (subscript
    (identifier) @type
  )
)

(type
  (identifier) @type
)

(function_definition
  name: (identifier) @function
)

(
  (call
    function: (identifier) @function.builtin
  )
  (#match? @function.builtin "^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$")
)

(
  (decorator
    (identifier) @attribute.builtin
  )
  (#match? @attribute.builtin "^(classmethod|property)$")
)

(decorator
  (call
    (attribute
      attribute: (identifier) @attribute
    )
  )
)

(decorator
  (call
    (identifier) @attribute
  )
)

(decorator
  (attribute
    attribute: (identifier) @attribute
  )
)

(decorator
  (identifier) @attribute
)

(
  (decorator
    "@" @attribute
  )
  (#set! "priority" 101)
)

(
  (call
    function: (attribute
      attribute: (identifier) @constructor
    )
  )
  (#match? @constructor "^[A-Z]")
)

(
  (call
    function: (identifier) @constructor
  )
  (#match? @constructor "^[A-Z]")
)

(call
  function: (attribute
    attribute: (identifier) @method.call
  )
)

(call
  function: (identifier) @function.call
)

(
  (assignment
    left: (identifier) @type.definition
    right: (call
      function: (identifier) @_func
    )
  )
  (#match? @_func "^(TypeVar|NewType)$")
)

(
  (assignment
    left: (identifier) @type.definition
    (type
      (identifier) @_annotation
    )
  )
  (#eq? @_annotation "TypeAlias")
)

(
  (attribute
    attribute: (identifier) @field
  )
  (#match? @field "^[[a-z]_][\\s\\S]*$")
)

(
  (identifier) @constant.builtin
  (#match? @constant.builtin "^(NotImplemented|Ellipsis|quit|exit|copyright|credits|license)$")
)

(
  (identifier) @constant.builtin
  (#match? @constant.builtin "^__[a-zA-Z0-9_]*__$")
)

(
  (identifier) @constant
  (#match? @constant "^[A-Z][A-Z_0-9]*$")
)

(
  (identifier) @type
  (#match? @type "^[A-Z][\\s\\S]*[a-z]")
)

(interpolation) @none

(identifier) @variable
