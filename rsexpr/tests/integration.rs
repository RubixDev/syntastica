#[test]
fn main() {
    let input = include_str!("/home/silas/Coding/Rust/syntastica/queries/html_tags/injections.scm");
    println!(
        "{}",
        rsexpr::from_slice_multi(&input)
            .unwrap()
            .into_iter()
            .map(|sexp| format!("{sexp:#}"))
            .collect::<Vec<_>>()
            .join("\n\n\n")
    );

    // let tree = &rsexpr::from_slice_multi(&input);
    // dbg!(tree);
}
