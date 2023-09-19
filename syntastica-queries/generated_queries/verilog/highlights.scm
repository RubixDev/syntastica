(ERROR) @error

[
  "["
  "]"
  "("
  ")"
] @punctuation.bracket

(generate_block_identifier) @comment

(type_declaration
  (simple_identifier) @type
)

(struct_union_member
  (data_type_or_void
    (data_type
      (simple_identifier) @type
    )
  )
)

(member_identifier
  (simple_identifier) @field
)

(struct_union_member
  (list_of_variable_decl_assignments
    (variable_decl_assignment
      (simple_identifier) @field
    )
  )
)

[
  (integer_atom_type)
  (non_integer_type)
  "genvar"
] @type.builtin

(type_declaration
  (simple_identifier) @type
)

(enum_name_declaration
  (enum_identifier
    (simple_identifier) @constant
  )
)

["enum"] @type

(struct_union) @type

(type_declaration
  (data_type
    ["packed"] @type.qualifier
  )
)

(task_identifier
  (task_identifier
    (simple_identifier) @method
  )
)

(function_subroutine_call
  (subroutine_call
    (system_tf_call
      (system_tf_identifier) @function.builtin
    )
  )
)

(function_subroutine_call
  (subroutine_call
    (tf_call
      (simple_identifier) @function
    )
  )
)

(function_identifier
  (function_identifier
    (simple_identifier) @function
  )
)

(lifetime) @label

(net_declaration
  (simple_identifier) @type
)

(interface_port_declaration
  (interface_identifier
    (simple_identifier) @type
  )
)

(name_of_instance
  (instance_identifier
    (simple_identifier) @variable
  )
)

(module_instantiation
  (simple_identifier) @constructor
)

(checker_instantiation
  (checker_identifier
    (simple_identifier) @constructor
  )
)

(time_unit) @attribute

[
  (integral_number)
  (unsigned_number)
  (unbased_unsized_literal)
] @number

(parameter_identifier
  (simple_identifier) @parameter
)

(class_constructor_declaration
  "new" @constructor
)

(module_declaration
  (module_header
    (simple_identifier) @constructor
  )
)

(text_macro_identifier
  (simple_identifier) @constant
)

(default_nettype_compiler_directive
  (default_nettype_value) @string
)

[
  ";"
  "::"
  ","
  "."
] @punctuation.delimiter

(seq_block
  (simple_identifier) @comment
)

(include_compiler_directive) @include

[
  (default_nettype_compiler_directive)
  (timescale_compiler_directive)
] @preproc

[
  (double_quoted_string)
  (string_literal)
] @string @spell

(net_port_type1
  (simple_identifier) @type
)

(modport_identifier
  (modport_identifier
    (simple_identifier) @field
  )
)

(interface_identifier
  (simple_identifier) @type
)

(method_call_body
  (method_identifier) @field
)

(data_type
  (simple_identifier) @type
)

[
  "signed"
  "unsigned"
] @type.qualifier

[
  (net_type)
  (integer_vector_type)
  (integer_atom_type)
] @type.builtin

(port_identifier
  (simple_identifier) @variable
)

(port_direction) @label

(edge_identifier) @attribute

(cast
  [
    "'"
    "("
    ")"
  ] @operator
)

[
  "or"
  "and"
] @keyword.operator

[
  "="
  "-"
  "+"
  "/"
  "*"
  "^"
  "&"
  "|"
  "&&"
  "||"
  ":"
  "{"
  "}"
  "'{"
  "<="
  "@"
  "=="
  "!="
  "==="
  "!=="
  "-:"
  "<"
  ">"
  ">="
  "%"
  ">>"
  "<<"
  "|="
  (unary_operator)
  (inc_or_dec_operator)
] @operator

(parameter_port_list
  "#" @constructor
)

(package_declaration
  (package_identifier
    (simple_identifier) @constant
  )
)

(package_scope
  (package_identifier
    (simple_identifier) @constant
  )
)

(text_macro_identifier
  (simple_identifier) @constant.macro
)

(package_import_declaration
  (package_import_item
    (package_identifier
      (simple_identifier) @constant
    )
  )
)

(package_import_declaration
  "import" @include
)

(include_compiler_directive) @constant.macro

(comment) @comment @spell

[
  "if"
  "else"
  (case_keyword)
  "endcase"
] @conditional

[
  (always_keyword)
  "generate"
  "for"
  "foreach"
  "repeat"
  "forever"
  "initial"
  "while"
] @repeat

[
  "begin"
  "end"
] @label

"return" @keyword.return

[
  "function"
  "endfunction"
  "task"
  "endtask"
] @keyword.function

[
  (module_keyword)
  "endmodule"
  "program"
  "endprogram"
  "class"
  "endclass"
  "interface"
  "endinterface"
  "package"
  "endpackage"
  "checker"
  "endchecker"
  "config"
  "endconfig"
  "pure"
  "virtual"
  "extends"
  "implements"
  "super"
  (class_item_qualifier)
  "parameter"
  "localparam"
  "defparam"
  "assign"
  "typedef"
  "modport"
  "fork"
  "join"
  "join_none"
  "join_any"
  "default"
  "break"
  "assert"
  "tagged"
  "extern"
  (unique_priority)
] @keyword
