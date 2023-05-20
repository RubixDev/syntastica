;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/go/injections.scm
;; Licensed under the Apache License 2.0

((comment) @injection.content
 (#set! injection.language "comment"))

(call_expression
  (selector_expression) @_function
  (#any-of? @_function
    "regexp.Match"
    "regexp.MatchReader"
    "regexp.MatchString"
    "regexp.Compile"
    "regexp.CompilePOSIX"
    "regexp.MustCompile"
    "regexp.MustCompilePOSIX")
  (argument_list
    . [(raw_string_literal) (interpreted_string_literal)] @injection.content
    (#offset! @injection.content 0 1 0 -1))
  (#set! injection.language "regex"))
