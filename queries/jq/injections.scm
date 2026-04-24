;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/jq/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

; test(val)
(query
  (
    (funcname) @_function
    (#any-of?
      @_function
      "test"
      "match"
      "capture"
      "scan"
      "split"
      "splits"
      "sub"
      "gsub"
    )
  )
  (args
    .
    (query
      (string) @injection.content
      (#set! injection.language "regex")
    )
  )
)

; test(regex; flags)
(query
  (
    (funcname) @_function
    (#any-of?
      @_function
      "test"
      "match"
      "capture"
      "scan"
      "split"
      "splits"
      "sub"
      "gsub"
    )
  )
  (args
    .
    (args
      (query
        (string) @injection.content
        (#set! injection.language "regex")
      )
    )
  )
)
