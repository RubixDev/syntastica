;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/java/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (block_comment)
    (line_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

 @markup.raw(
  (block_comment) @injection.content
  (#lua-match? @injection.content "/[*][*][%s]")
  (#set! injection.language "javadoc")
)

; markdown-style javadocs https://openjdk.org/jeps/467
(
  (line_comment) @injection.content
  (#lua-match? @injection.content "^///%s")
  (#set! injection.language "javadoc")
)

(
  (method_invocation
    name: (identifier) @_method
    arguments: (argument_list
      .
      (string_literal
        .
        (_) @injection.content
      )
    )
  )
  (#any-of? @_method "format" "printf")
  (#set! injection.language "printf")
)

(
  (method_invocation
    object: (string_literal
      (string_fragment) @injection.content
    )
    name: (identifier) @_method
  )
  (#eq? @_method "formatted")
  (#set! injection.language "printf")
)
