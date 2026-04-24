;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ssh_config/locals.scm
;; Licensed under the Apache License 2.0
(parameter
  keyword:
  "Tag"
  argument: (string) @local.reference
)

(condition
  criteria:
  "tagged"
  argument: (pattern) @local.definition
)
