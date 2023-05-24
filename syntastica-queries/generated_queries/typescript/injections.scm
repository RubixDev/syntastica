(
  (
    (comment) @_jsdoc_comment
    (#match? @_jsdoc_comment "^/[*][*][^*].*[*]/$")
  ) @injection.content
  (#set! injection.language "jsdoc")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(call_expression
  function: (
    (identifier) @injection.language
  )
  arguments: [
    (arguments
      (template_string) @injection.content
    )
    (template_string) @injection.content
  ]
  (#offset! @injection.content 0 1 0 -1)
  (#not-eq? @injection.language "svg")
)

(call_expression
  function: (
    (identifier) @_name
    (#eq? @_name "svg")
  )
  arguments: [
    (arguments
      (template_string) @injection.content
    )
    (template_string) @injection.content
  ]
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "html")
)

(call_expression
  function: (
    (identifier) @_name
    (#eq? @_name "gql")
  )
  arguments: (
    (template_string) @injection.content
    (#offset! @injection.content 0 1 0 -1)
    (#set! injection.language "graphql")
  )
)

(call_expression
  function: (
    (identifier) @_name
    (#eq? @_name "hbs")
  )
  arguments: (
    (template_string) @injection.content
    (#offset! @injection.content 0 1 0 -1)
    (#set! injection.language "glimmer")
  )
)

(
  (glimmer_template) @injection.content
  (#set! injection.language "glimmer")
)

(call_expression
  function: (member_expression
    object: (identifier) @_name
    (#eq? @_name "styled")
  )
  arguments: (
    (template_string) @injection.content
    (#offset! @injection.content 0 1 0 -1)
    (#set! injection.language "css")
  )
)

(call_expression
  function: (call_expression
    function: (identifier) @_name
    (#eq? @_name "styled")
  )
  arguments: (
    (template_string) @injection.content
    (#offset! @injection.content 0 1 0 -1)
    (#set! injection.language "css")
  )
)

(call_expression
  function: (call_expression
    function: (member_expression
      object: (member_expression
        object: (identifier) @_name
        (#eq? @_name "styled")
      )
    )
  )
  arguments: (
    (template_string) @injection.content
    (#offset! @injection.content 0 1 0 -1)
    (#set! injection.language "css")
  )
)

(call_expression
  function: (call_expression
    function: (member_expression
      object: (call_expression
        function: (identifier) @_name
        (#eq? @_name "styled")
      )
    )
  )
  arguments: (
    (template_string) @injection.content
    (#offset! @injection.content 0 1 0 -1)
    (#set! injection.language "css")
  )
)

(
  (regex_pattern) @injection.content
  (#set! injection.language "regex")
)

(
  (template_string) @injection.content
  (#match? @injection.content "^`#graphql")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "graphql")
)

(assignment_expression
  left: (member_expression
    property: (property_identifier) @_prop
    (#match? @_prop "^(innerHTML|outerHTML)$")
  )
  right: (template_string) @injection.content
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "html")
)

(assignment_expression
  left: (member_expression
    property: (property_identifier) @_prop
    (#match? @_prop "^(innerHTML|outerHTML)$")
  )
  right: (string) @injection.content
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "html")
)
