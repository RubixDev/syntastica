;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hyprlang/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(exec
  (string) @injection.content
  (#set! injection.language "bash")
)

(
  (keyword
    (name) @_bind
    (params
      .
      (_)
      .
      (_)
      .
      (_)?
      .
      (string) @_exec
      .
      (string) @injection.content
    )
  )
  (#lua-match? @_bind "^bind")
  (#lua-match? @_exec "^%s*exec%s*$")
  (#set! injection.language "bash")
)

(
  (assignment
    (name) @_name
    (string) @injection.content
  )
  (#any-of?
    @_name
    "lock_cmd"
    "unlock_cmd"
    "before_sleep_cmd"
    "after_sleep_cmd"
    "on-timeout"
    "on-resume"
    "reload_cmd"
  )
  (#set! injection.language "bash")
)
