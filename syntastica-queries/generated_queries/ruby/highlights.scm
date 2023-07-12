(ERROR) @error

(interpolation
  "#{" @punctuation.special
  "}" @punctuation.special
) @none

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "%w("
  "%i("
] @punctuation.bracket

[
  ","
  ";"
  "."
] @punctuation.delimiter

[
  "!"
  "="
  "=="
  "==="
  "<=>"
  "=>"
  "->"
  ">>"
  "<<"
  ">"
  "<"
  ">="
  "<="
  "**"
  "*"
  "/"
  "%"
  "+"
  "-"
  "&"
  "|"
  "^"
  "&&"
  "||"
  "||="
  "&&="
  "!="
  "%="
  "+="
  "-="
  "*="
  "/="
  "=~"
  "!~"
  "?"
  ":"
  ".."
  "..."
] @operator

(string_content) @spell

(body_statement
  (comment)
  + @comment.documentation
  (method)
)

(class
  (comment)
  + @comment.documentation
  (body_statement
    (method)
  )
)

(module
  (comment)
  + @comment.documentation
  (body_statement
    (class)
  )
)

(program
  (comment)
  + @comment.documentation
  (class)
)

(comment) @comment @spell

(nil) @constant.builtin

[
  (true)
  (false)
] @boolean

(float) @float

(integer) @number

(escape_sequence) @string.escape

(regex) @string.regex

(pair
  key: (hash_key_symbol)
  ":" @constant
)

[
  (bare_symbol)
  (simple_symbol)
  (delimited_symbol)
  (hash_key_symbol)
] @symbol

[
  (heredoc_beginning)
  (heredoc_end)
] @constant

[
  (string)
  (bare_string)
  (subshell)
  (heredoc_body)
] @string

(
  (identifier) @function
  (#is-not? local)
)

(keyword_parameter
  (identifier) @parameter
)

(block_parameter
  (identifier) @parameter
)

(destructured_parameter
  (identifier) @parameter
)

(optional_parameter
  (identifier) @parameter
)

(hash_splat_parameter
  (identifier) @parameter
)

(splat_parameter
  (identifier) @parameter
)

(block_parameters
  (identifier) @parameter
)

(lambda_parameters
  (identifier) @parameter
)

(method_parameters
  (identifier) @parameter
)

[
  (self)
  (super)
] @variable.builtin

(
  (constant) @type
  (#match? @type "^[A-Z\\d_]+$")
)

(
  (identifier) @constant.builtin
  (#match? @constant.builtin "^__(callee|dir|id|method|send|ENCODING|FILE|LINE)__$")
)

[
  (class_variable)
  (instance_variable)
] @label

(superclass
  (constant) @type
)

(module
  name: (constant) @type
)

(class
  name: (constant) @type
)

(singleton_method
  name: [
    (identifier) @function
    (constant) @type
  ]
)

(method
  name: [
    (identifier) @function
    (constant) @type
  ]
)

(setter
  (identifier) @function
)

(alias
  (identifier) @function
)

(program
  (call
    (identifier) @include
  )
  (#match? @include "^(require|require_relative|load)$")
)

(call
  receiver: (constant)
  ? @type
  method: [
    (identifier)
    (constant)
  ] @function.call
)

"defined?" @function

(
  (identifier) @exception
  (#match? @exception "^(fail|raise)$")
)

[
  "rescue"
  "ensure"
] @exception

(
  (identifier) @type.qualifier
  (#match? @type.qualifier "^(private|protected|public)$")
)

(constant) @type

[
  "for"
  "until"
  "while"
  "break"
  "redo"
  "retry"
  "next"
] @repeat

(if
  "end" @conditional
)

[
  "case"
  "else"
  "elsif"
  "if"
  "unless"
  "when"
  "then"
] @conditional

(method
  "end" @keyword.function
)

[
  "def"
  "undef"
] @keyword.function

[
  "and"
  "or"
  "in"
  "not"
] @keyword.operator

[
  "return"
  "yield"
] @keyword.return

[
  "alias"
  "begin"
  "class"
  "do"
  "end"
  "ensure"
  "module"
  "rescue"
  "then"
] @keyword

(global_variable) @variable.global

(identifier) @variable
