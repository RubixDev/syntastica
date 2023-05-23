(
  (program
    .
    (comment) @preproc
  )
  (#match? @preproc "^#!/")
)

(
  (regex) @string.regex
)

(case_item
  value: (word) @parameter
)

(
  (variable_name) @constant
  (#match? @constant "^[A-Z][A-Z_0-9]*$")
)

(
  (variable_name) @variable
)

(expansion
  [
    "${"
    "}"
  ] @punctuation.bracket
)

(file_redirect
  descriptor: (file_descriptor) @operator
  destination: (word) @parameter
)

(
  (word) @number
  (#match? @number "^[0-9]+$")
)

(command
  argument: [
    (word) @parameter
    (concatenation
      (word) @parameter
    )
  ]
)

(
  (command_name
    (word) @function.builtin
  )
  (#match? @function.builtin "^(alias|cd|clear|echo|eval|exit|getopts|popd|pushd|return|set|shift|shopt|source|test)$")
)

(command_name
  (word) @function.call
)

(function_definition
  name: (word) @function
)

(process_substitution
  [
    "<("
    ")"
  ] @punctuation.bracket
)

(command_substitution
  [
    "$("
    ")"
  ] @punctuation.bracket
)

(
  (test_operator) @string
)

(
  (comment) @comment @spell
)

(
  (word) @boolean
  (#match? @boolean "^(true|false)$")
)

(
  (word) @constant.builtin
  (#match? @constant.builtin "^SIG(HUP|INT|QUIT|ILL|TRAP|ABRT|BUS|FPE|KILL|USR[12]|SEGV|PIPE|ALRM|TERM|STKFLT|CHLD|CONT|STOP|TSTP|TT(IN|OU)|URG|XCPU|XFSZ|VTALRM|PROF|WINCH|IO|PWR|SYS|RTMIN([+]([1-9]|1[0-5]))?|RTMAX(-([1-9]|1[0-4]))?)$")
)

(
  (special_variable_name) @constant
)

("function"
  @keyword.function
)

(
  [
    "declare"
    "export"
    "local"
    "readonly"
    "unset"
  ] @keyword
)

(
  [
    "for"
    "do"
    "done"
    "while"
  ] @repeat
)

(
  [
    "if"
    "then"
    "else"
    "elif"
    "fi"
    "case"
    "in"
    "esac"
  ] @conditional
)

(variable_assignment
  (word) @string
)

(
  [
    (string)
    (raw_string)
    (ansi_c_string)
    (heredoc_body)
  ] @string @spell
)

(
  [
    ">"
    ">>"
    "<"
    "<<"
    "&"
    "&&"
    "|"
    "||"
    "="
    "=~"
    "=="
    "!="
  ] @operator
)

(
  ["$"] @punctuation.special
)

(
  [
    ";"
    ";;"
    (heredoc_start)
  ] @punctuation.delimiter
)

(
  [
    "("
    ")"
    "(("
    "))"
    "{"
    "}"
    "["
    "]"
    "[["
    "]]"
  ] @punctuation.bracket
)

(
  (expansion
    "${" @punctuation.special
    "}" @punctuation.special
  ) @none
)

(
  (simple_expansion) @none
)
