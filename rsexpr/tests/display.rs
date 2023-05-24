#[test]
#[cfg(feature = "comments")]
fn comments() {
    assert_eq!(
        format!(
            "{:#}",
            rsexpr::from_slice_multi(
                r###"
;; This is a comment
;; that spans multiple lines

(this is a tree)
["and" "another one"]
"###
            )
            .unwrap()
        ),
        r###";; This is a comment
;; that spans multiple lines
(this
  is
  a
  tree
)

[
  "and"
  "another one"
]
"###
    );

    assert_eq!(
        format!(
            "{:#}",
            rsexpr::from_slice_multi(
                r###"
(#predicate!
  ; this is a comment
)
"###
            )
            .unwrap()
        ),
        r###"
(#predicate!
  ; this is a comment
)
"###
        .trim_start()
    );

    assert_eq!(
        format!(
            "{:#}",
            rsexpr::from_slice_multi(
                r###"
"string" @string
(list) @list
["a" "b"] @group
"###
            )
            .unwrap()
        ),
        r###"
"string" @string

(list) @list

[
  "a"
  "b"
] @group
"###
        .trim_start()
    );
}
