(
  (function_call
    name: [
      (identifier) @_cdef_identifier
      (_
        _
        (identifier) @_cdef_identifier
      )
    ]
    arguments: (arguments
      (string
        content:
        _ @injection.content
        (#set! injection.language "c")
      )
    )
  )
  (#eq? @_cdef_identifier "cdef")
)

(
  (function_call
    name: (_) @_vimcmd_identifier
    arguments: (arguments
      .
      (string
        content:
        _ @injection.content
        (#set! injection.language "vim")
      )
    )
  )
  (#match? @_vimcmd_identifier "^(vim.cmd|vim.api.nvim_command|vim.api.nvim_exec|vim.api.nvim_exec2)$")
)

(
  (function_call
    name: (_) @_vimcmd_identifier
    arguments: (arguments
      (string
        content:
        _ @injection.content
        (#set! injection.language "query")
      )
      .
    )
  )
  (#match? @_vimcmd_identifier "^(vim.treesitter.query.set|vim.treesitter.query.parse_query|vim.treesitter.query.parse)$")
)

(
  (function_call
    name: (_) @_vimcmd_identifier
    arguments: (arguments
      .
      (_)
      .
      (string
        content:
        _ @_method
      )
      .
      (string
        content:
        _ @injection.content
        (#set! injection.language "lua")
      )
    )
  )
  (#match? @_vimcmd_identifier "^(vim.rpcrequest|vim.rpcnotify)$")
  (#eq? @_method "nvim_exec_lua")
)

(string
  content:
  _ @injection.content
  (#set! injection.language "query")
  (#match? @injection.content "^[ \\t\\n\\v\\f\\r]*;+[ \\t\\n\\v\\f\\r]?query")
)

(
  (comment) @injection.content
  (#set! injection.language "luadoc")
  (#match? @injection.content "[-][-][-][[ \\t\\n\\v\\f\\r]]*@")
  (#offset! @injection.content 0 3 0 0)
)

(function_call
  (dot_index_expression
    field: (identifier) @_method
    (#match? @_method "^(find|match)$")
  )
  arguments: (arguments
    (_)
    .
    (string
      content:
      _ @injection.content
      (#set! injection.language "luap")
    )
  )
)

(function_call
  (dot_index_expression
    field: (identifier) @_method
    (#match? @_method "^(gmatch|gsub)$")
  )
  arguments: (arguments
    (_)
    (string
      content:
      _ @injection.content
      (#set! injection.language "luap")
    )
  )
)

(function_call
  (method_index_expression
    method: (identifier) @_method
    (#match? @_method "^(find|match)$")
  )
  arguments: (arguments
    .
    (string
      content:
      _ @injection.content
      (#set! injection.language "luap")
    )
  )
)

(function_call
  (method_index_expression
    method: (identifier) @_method
    (#match? @_method "^(gmatch|gsub)$")
  )
  arguments: (arguments
    (string
      content:
      _ @injection.content
      (#set! injection.language "luap")
    )
  )
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)
