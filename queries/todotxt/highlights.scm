;; Forked from https://raw.githubusercontent.com/arnarg/tree-sitter-todotxt/3937c5cd105ec4127448651a21aef45f52d19609/queries/highlights.scm
(done_task) @comment

(task
  (priority) @keyword
)

(task
  (date) @comment
)

(task
  (kv) @comment
)

(task
  (project) @string
)

(task
  (context) @type
)
