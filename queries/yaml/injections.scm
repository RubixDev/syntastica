;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/yaml/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

; Github actions ("run") / Gitlab CI ("scripts")
; Taskfile scripts ("cmds", "cmd", "sh")
(block_mapping_pair
  key: (flow_node) @_run
  (#any-of?
    @_run
    "run"
    "script"
    "before_script"
    "after_script"
    "cmds"
    "cmd"
    "sh"
  )
  value: (flow_node
    (plain_scalar
      (string_scalar) @injection.content
    )
    (#set! injection.language "bash")
  )
)

(block_mapping_pair
  key: (flow_node) @_run
  (#any-of?
    @_run
    "run"
    "script"
    "before_script"
    "after_script"
    "cmds"
    "cmd"
    "sh"
  )
  value: (block_node
    (block_scalar) @injection.content
    (#set! injection.language "bash")
    (#offset! @injection.content 0 1 0 0)
  )
)

(block_mapping_pair
  key: (flow_node) @_run
  (#any-of?
    @_run
    "run"
    "script"
    "before_script"
    "after_script"
    "cmds"
    "sh"
  )
  value: (block_node
    (block_sequence
      (block_sequence_item
        (flow_node
          (plain_scalar
            (string_scalar) @injection.content
          )
        )
        (#set! injection.language "bash")
      )
    )
  )
)

(block_mapping_pair
  key: (flow_node) @_run
  (#any-of? @_run "script" "before_script" "after_script" "cmds" "sh")
  value: (block_node
    (block_sequence
      (block_sequence_item
        (block_node
          (block_scalar) @injection.content
          (#set! injection.language "bash")
          (#offset! @injection.content 0 1 0 0)
        )
      )
    )
  )
)

; Prometheus Alertmanager ("expr")
(block_mapping_pair
  key: (flow_node) @_expr
  (#eq? @_expr "expr")
  value: (flow_node
    (plain_scalar
      (string_scalar) @injection.content
    )
    (#set! injection.language "promql")
  )
)

(block_mapping_pair
  key: (flow_node) @_expr
  (#eq? @_expr "expr")
  value: (block_node
    (block_scalar) @injection.content
    (#set! injection.language "promql")
    (#offset! @injection.content 0 1 0 0)
  )
)

(block_mapping_pair
  key: (flow_node) @_expr
  (#eq? @_expr "expr")
  value: (block_node
    (block_sequence
      (block_sequence_item
        (flow_node
          (plain_scalar
            (string_scalar) @injection.content
          )
        )
        (#set! injection.language "promql")
      )
    )
  )
)

(block_mapping_pair
  key: (flow_node) @_expr
  (#eq? @_expr "expr")
  value: (block_node
    (block_sequence
      (block_sequence_item
        (block_node
          (block_scalar) @injection.content
          (#set! injection.language "promql")
          (#offset! @injection.content 0 1 0 0)
        )
      )
    )
  )
)
