;; Forked from https://raw.githubusercontent.com/the-mikedavis/tree-sitter-git-rebase/bff4b66b44b020d918d67e2828eada1974a966aa/queries/highlights.scm
; a rough translation:
; * constant.builtin - git hash
; * constant - a git label
; * keyword - command that acts on commits commits
; * function - command that acts only on labels
; * comment - discarded commentary on a command, has no effect on the rebase
; * string - text used in the rebase operation
; * operator - a 'switch' (used in fixup and merge), either -c or -C at time of writing
(
  (
    (command) @keyword
    (label) @constant.builtin
    (message)? @comment
  )
  (#match? @keyword "^(p|pick|r|reword|e|edit|s|squash|d|drop)$")
)

(
  (
    (command) @function
    (label) @constant
    (message)? @comment
  )
  (#match? @function "^(l|label|t|reset)$")
)

(
  (command) @keyword
  (#match? @keyword "^(x|exec|b|break)$")
)

(
  (
    (command) @attribute
    (label) @constant.builtin
    (message)? @comment
  )
  (#match? @attribute "^(f|fixup)$")
)

(
  (
    (command) @keyword
    (label) @constant.builtin
    (label) @constant
    (message) @string
  )
  (#match? @keyword "^(m|merge)$")
)

(option) @operator

(comment) @comment
