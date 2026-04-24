;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hlsl/highlights.scm
;; Licensed under the Apache License 2.0
; inherits: cpp
[
  "in"
  "out"
  "inout"
  "uniform"
  "shared"
  "groupshared"
  "discard"
  "cbuffer"
  "row_major"
  "column_major"
  "globallycoherent"
  "centroid"
  "noperspective"
  "nointerpolation"
  "sample"
  "linear"
  "snorm"
  "unorm"
  "point"
  "line"
  "triangleadj"
  "lineadj"
  "triangle"
] @keyword.modifier

(
  (identifier) @variable.builtin
  (#lua-match? @variable.builtin "^SV_")
)

(hlsl_attribute) @attribute

(hlsl_attribute
  [
    "["
    "]"
  ] @attribute
)
