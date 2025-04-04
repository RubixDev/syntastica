;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/go/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(call_expression
  (selector_expression) @_function
  (#any-of?
    @_function
    "regexp.Match"
    "regexp.MatchReader"
    "regexp.MatchString"
    "regexp.Compile"
    "regexp.CompilePOSIX"
    "regexp.MustCompile"
    "regexp.MustCompilePOSIX"
  )
  (argument_list
    .
    [
      (raw_string_literal
        (raw_string_literal_content) @injection.content
      )
      (interpreted_string_literal
        (interpreted_string_literal_content) @injection.content
      )
    ]
    (#set! injection.language "regex")
  )
)

(
  (comment) @injection.content
  (#match? @injection.content "/\\*!([a-zA-Z]+:)?re2c")
  (#set! injection.language "re2c")
)

(
  (call_expression
    function: (selector_expression
      field: (field_identifier) @_method
    )
    arguments: (argument_list
      .
      (interpreted_string_literal
        (interpreted_string_literal_content) @injection.content
      )
    )
  )
  (#any-of?
    @_method
    "Printf"
    "Sprintf"
    "Fatalf"
    "Scanf"
    "Errorf"
    "Skipf"
    "Logf"
  )
  (#set! injection.language "printf")
)

(
  (call_expression
    function: (selector_expression
      field: (field_identifier) @_method
    )
    arguments: (argument_list
      (_)
      .
      (interpreted_string_literal
        (interpreted_string_literal_content) @injection.content
      )
    )
  )
  (#any-of? @_method "Fprintf" "Fscanf" "Appendf" "Sscanf")
  (#set! injection.language "printf")
)
