;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/python/injections.scm
;; Licensed under the Apache License 2.0
(
  (call
    function: (attribute
      object: (identifier) @_re
    )
    arguments: (argument_list
      (string) @regex
    )
  )
  (#eq? @_re "re")
  (#lua-match? @regex "^r.*")
)

(comment) @comment
