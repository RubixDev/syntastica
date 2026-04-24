;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/cuda/highlights.scm
;; Licensed under the Apache License 2.0
; inherits: cpp
[
  "<<<"
  ">>>"
] @punctuation.bracket

[
  "__host__"
  "__device__"
  "__global__"
  "__managed__"
  "__forceinline__"
  "__noinline__"
] @keyword.modifier

"__launch_bounds__" @keyword.modifier
