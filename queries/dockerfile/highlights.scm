;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/dockerfile/highlights.scm
;; Licensed under the Apache License 2.0
[
  "FROM"
  "AS"
  "RUN"
  "CMD"
  "LABEL"
  "EXPOSE"
  "ENV"
  "ADD"
  "COPY"
  "ENTRYPOINT"
  "VOLUME"
  "USER"
  "WORKDIR"
  "ARG"
  "ONBUILD"
  "STOPSIGNAL"
  "HEALTHCHECK"
  "SHELL"
  "MAINTAINER"
  "CROSS_BUILD"
] @keyword

[
  ":"
  "@"
] @operator

(comment) @comment

(image_spec
  (image_tag
    ":" @punctuation.special
  )
  (image_digest
    "@" @punctuation.special
  )
)

(json_string) @string

(double_quoted_string) @string

[
  (heredoc_marker)
  (heredoc_end)
] @label

(
  (heredoc_block
    (heredoc_line) @string
  )
  (#set! priority 90)
)

(expansion
  [
    "$"
    "{"
    "}"
  ] @punctuation.special
)

(
  (variable) @constant
  (#lua-match? @constant "^[A-Z][A-Z_0-9]*$")
)

(arg_instruction
  .
  (unquoted_string) @property
)

(env_instruction
  (env_pair
    .
    (unquoted_string) @property
  )
)

(expose_instruction
  (expose_port) @number
)

[
  "["
  "]"
] @punctuation.bracket

"," @punctuation.delimiter
