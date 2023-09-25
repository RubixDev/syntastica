use std::{fs, process::Command};

use anyhow::{bail, Context, Result};

pub fn run() -> Result<()> {
    let tmpfile = crate::WORKSPACE_DIR.join("ignored/tmp.typ");
    const ASSET_PATH: &str = "syntastica-themes/assets/theme-svgs";
    let out_dir = crate::WORKSPACE_DIR.join(ASSET_PATH);
    fs::create_dir_all(crate::WORKSPACE_DIR.join("ignored"))?;
    fs::create_dir_all(&out_dir)?;

    let mut md_list = String::from("## List of Themes\n");

    for &theme_name in syntastica_themes::THEMES {
        println!("{theme_name}");

        let theme = syntastica_themes::from_str(theme_name).unwrap();
        let bg = match theme.bg() {
            Some(color) => {
                let (r, g, b) = color.into_components();
                format!("rgb({r}, {g}, {b})")
            }
            None => "none".to_string(),
        };

        let output = Command::new(env!("CARGO"))
            .args(["run", "--example=custom_renderer", theme_name])
            .current_dir(&*crate::WORKSPACE_DIR)
            .output()?;
        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            bail!("cargo returned non-zero exit code: {}", output.status);
        }
        let render = String::from_utf8_lossy(&output.stdout);

        let typst_file = format!(
            r#"
#set page(height: auto, width: auto, margin: 8pt)
#show raw: text.with(font: "JetBrains Mono")

#block(fill: {bg}, inset: 8pt, radius: 5pt)[
  == *#box(
    fill: luma(240),
    inset: (x: 3pt, y: 0pt),
    outset: (y: 3pt),
    radius: 2pt,
    raw("{theme_name}", block: false),
  )*\
  {render}
]
"#
        );
        fs::write(&tmpfile, typst_file)?;

        let status = Command::new("typst")
            .args(["compile", "--format=svg", "--root"])
            .arg(&*crate::WORKSPACE_DIR)
            .arg(&tmpfile)
            .arg(out_dir.join(format!("{theme_name}.svg")))
            .current_dir(&*crate::WORKSPACE_DIR)
            .status()
            .with_context(|| "failed to run typst compiler")?;
        if !status.success() {
            bail!("typst exited with non-zero exit code: {status}");
        }

        md_list += &format!(
            r#"
### `{theme_name}`

<img alt="{theme_name}" width="400" src="https://github.com/RubixDev/syntastica/raw/main/{ASSET_PATH}/{theme_name}.svg"></img>
"#
        )
    }

    fs::write(
        crate::WORKSPACE_DIR.join("syntastica-themes/theme_list.md"),
        md_list,
    )?;

    Ok(())
}
