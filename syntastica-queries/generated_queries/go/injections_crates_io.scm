(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(call_expression
  (selector_expression) @_function
  (#match? @_function "^(regexp.Match|regexp.MatchReader|regexp.MatchString|regexp.Compile|regexp.CompilePOSIX|regexp.MustCompile|regexp.MustCompilePOSIX)$")
  (argument_list
    .
    [
      (raw_string_literal)
      (interpreted_string_literal)
    ] @injection.content
    (#set! injection.language "regex")
    (#offset! @injection.content 0 1 0 -1)
  )
)
