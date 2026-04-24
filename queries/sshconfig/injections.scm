;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ssh_config/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (condition
    criteria:
    "exec"
    argument: (string) @injection.content
  )
  (#set! injection.language "bash")
)

(
  (parameter
    keyword: [
      "KnownHostsCommand"
      "LocalCommand"
      "RemoteCommand"
      "ProxyCommand"
    ]
    argument: (string) @injection.content
  )
  (#set! injection.language "bash")
)
