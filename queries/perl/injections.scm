;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/perl/injections.scm
;; Licensed under the Apache License 2.0
; an injections.scm file for nvim-treesitter
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (pod) @injection.content
  (#set! injection.language "pod")
)

(
  (substitution_regexp
    (replacement) @injection.content
    (substitution_regexp_modifiers) @_modifiers
  )
  ; match if there's a single `e` in the modifiers list
  (#lua-match? @_modifiers "e")
  (#not-lua-match? @_modifiers "e.*e")
  (#set! injection.language "perl")
  (#set! injection.include-children)
)

(heredoc_content
  (heredoc_end) @injection.language
) @injection.content
