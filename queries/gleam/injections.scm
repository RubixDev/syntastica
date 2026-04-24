; crates.io skip
((doc_comment_content) @injection.content
 (#set! injection.language "markdown")
 (#set! injection.combined))

; non-crates.io skip
((module_comment) @injection.content
 (#set! injection.language "markdown")
 (#set! injection.combined))

; non-crates.io skip
((statement_comment) @injection.content
 (#set! injection.language "markdown")
 (#set! injection.combined))
