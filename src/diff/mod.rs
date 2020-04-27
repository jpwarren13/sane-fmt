use difference::Difference::*;
pub use difference::Changeset as Diff;

pub fn diff(old: &String, new: &String) -> Diff {
    Diff::new(old.as_str(), new.as_str(), "\n")
}

pub fn diff_text(old: &String, new: &String) -> String {
    diff(old, new).diffs
        .into_iter()
        .map(|diff| match diff {
            Same(line) => add_prefix(line, "   "),
            Add(line) => add_prefix(line, "  +"),
            Rem(line) => add_prefix(line, "  -"),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn add_prefix(text: String, prefix: &str) -> String {
    text.split("\n")
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}
