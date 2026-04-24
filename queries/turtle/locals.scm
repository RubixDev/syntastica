;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/turtle/locals.scm
;; Licensed under the Apache License 2.0
(document) @local.scope

(subject
  [
    (prefixed_name)
    (iri_reference)
  ] @local.definition.var
)

[
  (prefixed_name)
  (iri_reference)
] @local.reference
