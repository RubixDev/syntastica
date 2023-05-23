#[test]
#[cfg(feature = "comments")]
fn comments() {
    let input = r###"
;; This is a comment
;; that spans multiple lines

(this is a tree)
["and" "another one"]
"###;
    assert_eq!(
        format!("{:#}", rsexpr::from_slice_multi(input).unwrap()),
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
}
