([(line_comment)(multiline_comment)]@injection.content (#set! injection.language "comment"))(call_expression (navigation_expression ((string_literal)@injection.content (#set! injection.language "regex"))(navigation_suffix ((simple_identifier)@_function (#eq? @_function "toRegex")))))(call_expression ((simple_identifier)@_function (#eq? @_function "Regex"))(call_suffix (value_arguments (value_argument (string_literal)@injection.content (#set! injection.language "regex")))))(call_expression (navigation_expression ((simple_identifier)@_class (#eq? @_class "Regex"))(navigation_suffix ((simple_identifier)@_function (#eq? @_function "fromLiteral"))))(call_suffix (value_arguments (value_argument (string_literal)@injection.content (#set! injection.language "regex")))))((call_expression (navigation_expression (string_literal (string_content)@injection.content)(navigation_suffix (simple_identifier)@_method)))(#eq? @_method "format")(#set! injection.language "printf"))