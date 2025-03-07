use dioxus::prelude::*;
use syntastica::renderer::HtmlRenderer;
use syntastica_parsers_git::{Lang, LanguageSetImpl};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // on an actual website it would be smart to run syntsatica in a web worker as to not
    // freeze the user interface while waiting
    let html = use_hook(|| {
        syntastica::highlight(
            "fn main() {\n    println!(\"Hello, World!\");\n}",
            Lang::Rust,
            &LanguageSetImpl::new(),
            &mut HtmlRenderer,
            syntastica_themes::one::dark(),
        )
        .map_err(|e| e.to_string())
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        match html {
            Ok(html) => rsx! { div { class: "code", dangerous_inner_html: "{html}" } },
            Err(err) => rsx! { div { color: "red", "{err}" } },
        }
    }
}
