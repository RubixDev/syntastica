;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/glimmer_javascript/injections.scm
;; Licensed under the Apache License 2.0
; inherits: ecma
; Ember Unified <template> syntax
; e.g.: <template><SomeComponent @arg={{double @value}} /></template>
(
  (glimmer_template) @injection.content
  (#set! injection.language "glimmer")
  (#set! injection.include-children)
)
