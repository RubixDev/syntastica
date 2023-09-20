use dioxus::prelude::*;
use syntastica::renderer::HtmlRenderer;
use syntastica_parsers_git::{Lang, LanguageSetImpl};

fn main() {
    dioxus_web::launch(app)
}

fn app(cx: Scope) -> Element {
    // on an actual website it would be smart to run syntsatica in a web worker as to not
    // freeze the user interface while waiting
    let html = syntastica::highlight(
        "fn main() {\n    println!(\"Hello, World!\");\n}",
        Lang::Rust,
        &LanguageSetImpl::new(),
        &mut HtmlRenderer,
        syntastica_themes::one::dark(),
    );

    match html {
        Ok(html) => render! { div { class: "code", dangerous_inner_html: "{html}" } },
        Err(err) => render! { div { color: "red", "{err}" } },
    }
}
