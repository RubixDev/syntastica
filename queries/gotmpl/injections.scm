;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gotmpl/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

; {{"put" | printf "%s%s" "out" | printf "%q"}}
(function_call
  function: (identifier) @_function
  arguments: (argument_list
    .
    (interpreted_string_literal) @injection.content
  )
  (#eq? @_function "printf")
  (#set! injection.language "printf")
)

; {{ js "var a = 1 + 1" }}
(function_call
  function: (identifier) @_function
  arguments: (argument_list
    .
    (interpreted_string_literal) @injection.content
  )
  (#eq? @_function "js")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "javascript")
)

; {{ html "<h1>hello</h1>" }}
(function_call
  function: (identifier) @_function
  arguments: (argument_list
    .
    (interpreted_string_literal) @injection.content
  )
  (#eq? @_function "html")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "html")
)
