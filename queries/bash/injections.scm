;; Forked from https://github.com/helix-editor/helix/blob/master/runtime/queries/bash/injections.scm
;; Licensed under the Mozilla Public License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (regex) @injection.content
  (#set! injection.language "regex")
)

(command
  name: (command_name
    (word) @_command
  )
  argument: (raw_string) @injection.content
  (#match? @_command "^[gnm]?awk$")
  (#set! injection.language "awk")
)
