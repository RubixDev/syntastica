(ERROR) @error

[
  "undef"
  "poison"
  "null"
  "none"
  "zeroinitializer"
] @constant.builtin

[
  "true"
  "false"
] @boolean

[
  "="
  "|"
  "x"
  "..."
] @operator

[
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<"
  ">"
  "<{"
  "}>"
] @punctuation.bracket

[
  (struct_value)
  (array_value)
  (vector_value)
] @constructor

(float) @float

(_
  inst_name:
  "ret" @keyword.return
)

(label) @label

(cstring) @string

(string) @string

(comment) @comment

(number) @number

(function_header
  [
    (linkage)
    (calling_conv)
    (unnamed_addr)
  ] @keyword.function
)

(attribute_name) @attribute

[
  "thread_local"
  "localdynamic"
  "initialexec"
  "localexec"
  (unnamed_addr)
  (dll_storage_class)
] @storageclass

[
  "no_cfi"
  (dso_local)
  (linkage_aux)
  (visibility)
] @type.qualifier

[
  "target"
  "triple"
  "datalayout"
  "source_filename"
  "addrspace"
  "blockaddress"
  "align"
  "syncscope"
  "within"
  "uselistorder"
  "uselistorder_bb"
  "module"
  "asm"
  "sideeffect"
  "alignstack"
  "inteldialect"
  "unwind"
  "type"
  "global"
  "constant"
  "externally_initialized"
  "alias"
  "ifunc"
  "section"
  "comdat"
  "any"
  "exactmatch"
  "largest"
  "nodeduplicate"
  "samesize"
  "distinct"
  "attributes"
  "vscale"
] @keyword

(calling_conv) @keyword.function

[
  "declare"
  "define"
] @keyword.function

(function_header
  name:
  _ @function
)

(_
  callee:
  _ @function
)

(fast_math) @keyword

(fcmp_cond) @keyword

(icmp_cond) @keyword

[
  "to"
  "nuw"
  "nsw"
  "exact"
  "unwind"
  "from"
  "cleanup"
  "swifterror"
  "volatile"
  "inbounds"
  "inrange"
] @keyword

[
  "catch"
  "filter"
] @keyword.operator

(_
  inst_name:
  _ @keyword.operator
)

(argument) @parameter

(global_type
  (local_var) @type.definition
)

(type
  [
    (local_var)
    (global_var)
  ] @type
)

(type_keyword) @type.builtin

(type) @type

[
  (local_var)
  (global_var)
] @variable
