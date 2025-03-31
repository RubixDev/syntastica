;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/diff/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment

[
  (addition)
  (new_file)
] @diff.plus

[
  (deletion)
  (old_file)
] @diff.minus

(commit) @constant

(location) @attribute

(command
  "diff" @function
  (argument) @variable.parameter
)

(filename) @string.special.path

(mode) @number

(
  [
    ".."
    "+"
    "++"
    "+++"
    "++++"
    "-"
    "--"
    "---"
    "----"
  ] @punctuation.special
  (#set! priority 95)
)

[
  (binary_change)
  (similarity)
  (file_change)
] @label

(index
  "index" @keyword
)

(similarity
  (score) @number
  "%" @number
)
