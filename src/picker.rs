use ignore::Walk;
use anyhow::{Context, Result};
use skim::prelude::*;
use std::io::Cursor;

pub fn pick_files() -> Result<Vec<String>> {
    let files: Vec<String> = Walk::new(".")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let fname = path.file_name()?.to_str()?;
            let ext = path.extension()?.to_str()?;

            if fname.contains(".test.") && matches!(ext, "js" | "jsx" | "ts" | "tsx") {
                let s = path.display().to_string();
                Some(s.strip_prefix("./").unwrap_or(&s).to_string())
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        anyhow::bail!("No Jest test files found.");
    }

    let options = SkimOptionsBuilder::default()
        .multi(true)
        .preview(Some(
            "bat --color=always --style=numbers {} 2>/dev/null || cat {}".into(),
        ))
        .prompt("jesty> ".to_string())
        .build()
        .context("failed to build skim options")?;

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(files.join("\n")));

    let selected = Skim::run_with(&options, Some(items))
        .filter(|out| !out.is_abort)
        .map(|out| out.selected_items);

    selected
        .map(|items| items.iter().map(|i| i.output().to_string()).collect())
        .context("no files selected")
}
