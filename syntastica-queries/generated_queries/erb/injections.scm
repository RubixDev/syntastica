(
  (content) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(directive
  (code) @injection.content
  (#set! injection.language "ruby")
  (#set! injection.combined)
)

(output_directive
  (code) @injection.content
  (#set! injection.language "ruby")
  (#set! injection.combined)
)

(graphql_directive
  (code) @injection.content
  (#set! injection.language "graphql")
)
