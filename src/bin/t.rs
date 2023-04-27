use std::fs;

use anyhow::Context;
use corrode::parsing::Frontmatter;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./content/test/test.md")
        .with_context(|| "Error opening 'content/test/test.md'.")?;
    let (fm, md) = Frontmatter::parse(&input).with_context(|| "Could not parse frontmatter.")?;
    dbg!(fm);
    println!("{md}");

    Ok(())
}
