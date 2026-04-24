;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/udev/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (match
    key:
    "PROGRAM"
    (value
      (content) @injection.content
    )
  )
  (#set! injection.language "bash")
)

(
  (assignment
    key:
    "RUN"
    (value
      (content) @injection.content
    )
  )
  (#set! injection.language "bash")
)
