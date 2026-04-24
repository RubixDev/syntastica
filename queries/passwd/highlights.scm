;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/passwd/highlights.scm
;; Licensed under the Apache License 2.0
(user) @module

(auth) @string.special.symbol

(gecos) @string

(home) @string.special.path

(shell) @string.special.path

[
  (gid)
  (uid)
] @number

(separator) @punctuation.delimiter
