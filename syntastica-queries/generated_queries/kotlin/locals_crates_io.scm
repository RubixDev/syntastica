(package_header . (identifier)@local.definition)(import_header (identifier (simple_identifier)@local.definition .)(import_alias (type_identifier)@local.definition)?)(function_declaration . (simple_identifier)@local.definition (#set! definition.function.scope "parent"))(class_body (function_declaration . (simple_identifier)@local.definition)(#set! definition.method.scope "parent"))(function_declaration (function_value_parameters (parameter (simple_identifier)@local.definition)))(lambda_literal (lambda_parameters (variable_declaration (simple_identifier)@local.definition)))(class_declaration (primary_constructor (class_parameter (simple_identifier)@local.definition)))(enum_class_body (enum_entry (simple_identifier)@local.definition))(variable_declaration (simple_identifier)@local.definition)(class_declaration (type_identifier)@local.definition (#set! definition.type.scope "parent"))(type_alias (type_identifier)@local.definition (#set! definition.type.scope "parent"))[(if_expression)(when_expression)(when_entry)(for_statement)(while_statement)(do_while_statement)(lambda_literal)(function_declaration)(primary_constructor)(secondary_constructor)(anonymous_initializer)(class_declaration)(enum_class_body)(enum_entry)(interpolated_expression)]@local.scope (simple_identifier)@local.reference 