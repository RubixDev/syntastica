;; Forked from https://raw.githubusercontent.com/savonet/tree-sitter-liquidsoap/d092c65598e940564e12214ff7c2097528cfa159/queries/highlights.scm
(op) @keyword.operator

"..." @keyword.operator

[
  "open"
  "def"
  "let"
  "if"
  "then"
  "elsif"
  "else"
  "begin"
  "end"
  "while"
  "do"
  "to"
  "for"
  "fun"
  "try"
  "catch"
] @keyword

[
  "and"
  "or"
] @conditional

(inline_if
  [
    "?"
    ":"
  ] @conditional.ternary
)

[
  "%ifdef"
  "%ifndef"
  "%ifencoder"
  "%ifnencoder"
  "%ifversion"
  "%else"
  "%endif"
  "%argsof"
  "%include"
] @preproc

(encoder_name) @constant.builtin

(anonymous_argument
  (var) @parameter
)

(labeled_argument
  label: (var) @parameter
)

"." @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(app
  name: (var) @function.call
)

(method) @method

(method_app) @method.call

(method) @function.method

(var) @variable

(string) @string

(integer) @number

(float) @number

(bool) @boolean

(
  (var) @constant.builtin
  (#eq? @constant.builtin "null")
)

(comment) @comment

(regexp) @string.regex

(type) @type
