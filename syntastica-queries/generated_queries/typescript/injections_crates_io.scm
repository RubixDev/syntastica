(((comment)@_jsdoc_comment (#match? @_jsdoc_comment "^\\/[\\*][\\*][^\\*][\\s\\S]*[\\*]\\/$"))@injection.content (#set! injection.language "jsdoc"))((comment)@injection.content (#set! injection.language "comment"))(call_expression function: (identifier)@injection.language arguments: [(arguments (template_string)@injection.content)(template_string)@injection.content](#match? @injection.language "^[a-zA-Z][a-zA-Z0-9]*$")(#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#not-match? @injection.language "^(svg|css)$"))(call_expression function: (identifier)@_name (#eq? @_name "svg")arguments: [(arguments (template_string)@injection.content)(template_string)@injection.content](#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "html"))(call_expression function: (member_expression property: (property_identifier)@injection.language)arguments: [(arguments (template_string)@injection.content)(template_string)@injection.content](#eq? @injection.language "sql")(#offset! @injection.content 0 1 0 -1)(#set! injection.include-children))(call_expression function: (identifier)@_name (#eq? @_name "gql")arguments: (template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "graphql"))(call_expression function: (identifier)@_name (#eq? @_name "hbs")arguments: (template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "glimmer"))(call_expression function: (identifier)@_name (#match? @_name "^(css|keyframes)$")arguments: (template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "styled"))(call_expression function: (member_expression object: (identifier)@_name (#eq? @_name "styled"))arguments: ((template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "styled")))(call_expression function: (call_expression function: (identifier)@_name (#eq? @_name "styled"))arguments: ((template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "styled")))(call_expression function: (call_expression function: (member_expression object: (member_expression object: (identifier)@_name (#eq? @_name "styled"))))arguments: ((template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "styled")))(call_expression function: (call_expression function: (member_expression object: (call_expression function: (identifier)@_name (#eq? @_name "styled"))))arguments: ((template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "styled")))((regex_pattern)@injection.content (#set! injection.language "regex"))((template_string)@injection.content (#match? @injection.content "^`#graphql")(#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "graphql"))(assignment_expression left: (member_expression property: (property_identifier)@_prop (#match? @_prop "^(outerHTML|innerHTML)$"))right: (template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "html"))(assignment_expression left: (member_expression property: (property_identifier)@_prop (#match? @_prop "^(outerHTML|innerHTML)$"))right: (string (string_fragment)@injection.content)(#set! injection.language "html"))(decorator (call_expression function: ((identifier)@_name (#eq? @_name "Component"))arguments: (arguments (object (pair key: ((property_identifier)@_prop (#eq? @_prop "template"))value: ((template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "angular")))))))(decorator (call_expression function: ((identifier)@_name (#eq? @_name "Component"))arguments: (arguments (object (pair key: ((property_identifier)@_prop (#eq? @_prop "styles"))value: (array ((template_string)@injection.content (#offset! @injection.content 0 1 0 -1)(#set! injection.include-children)(#set! injection.language "css"))))))))(decorator (call_expression function: ((identifier)@_name (#eq? @_name "Component"))arguments: (arguments (object (pair key: ((property_identifier)@_prop (#eq? @_prop "styles"))value: ((template_string)@injection.content (#set! injection.include-children)(#offset! @injection.content 0 1 0 -1)(#set! injection.language "css")))))))