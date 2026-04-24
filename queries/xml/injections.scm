;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/xml/injections.scm
;; Licensed under the Apache License 2.0
(
  (Comment) @injection.content
  (#set! injection.language "comment")
)

; SVG style
(
  (element
    (STag
      (Name) @_name
    )
    (content) @injection.content
  )
  (#eq? @_name "style")
  (#set! injection.combined)
  (#set! injection.include-children)
  (#set! injection.language "css")
)

; SVG script
(
  (element
    (STag
      (Name) @_name
    )
    (content) @injection.content
  )
  (#eq? @_name "script")
  (#set! injection.combined)
  (#set! injection.include-children)
  (#set! injection.language "javascript")
)

; phpMyAdmin dump
(
  (element
    (STag
      (Name) @_name
    )
    (content) @injection.content
  )
  (#eq? @_name "pma:table")
  (#set! injection.combined)
  (#set! injection.include-children)
  (#set! injection.language "sql")
)
