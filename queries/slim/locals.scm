;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/slim/locals.scm
;; Licensed under the Apache License 2.0
(child) @local.scope

(tag_name) @local.definition.function

(tag_class) @local.definition.parameter

(tag_id) @local.definition.constant
