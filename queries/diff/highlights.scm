;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/diff/highlights.scm
;; Licensed under the Apache License 2.0
[
  (addition)
  (new_file)
] @text.diff.add

[
  (deletion)
  (old_file)
] @text.diff.delete

(commit) @constant

(location) @attribute

(command) @function
