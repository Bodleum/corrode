use nom::{
    bytes::complete::{tag, take},
    character::complete::{line_ending, multispace0},
    combinator::{not, recognize},
    error::context,
    multi::many0_count,
    sequence::{delimited, terminated, tuple},
    Finish,
};
use serde::{Deserialize, Serialize};

use crate::error::FrontmatterError;

use super::{IResult, Span};

#[derive(Debug, Serialize, Deserialize)]
pub struct Frontmatter {
    title: String,
    created: chrono::NaiveDate,
    modified: chrono::NaiveDate,
    tags: Vec<String>,
}

impl Frontmatter {
    pub fn parse(input: &str) -> Result<(Self, &str), FrontmatterError> {
        let marker = "---";
        let (input, _) =
            context("Finding first marker", match_marker(marker))(Span::new(&input)).finish()?;
        let (input, fm) = grab_frontmatter(marker)(input).finish()?;
        let (md, _) = context("Finding second marker", match_marker(marker))(input).finish()?;

        Ok((toml::from_str(fm.fragment())?, *md.fragment()))
    }
}

// Parser helpers
fn ws<'a, F>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>>
where
    F: FnMut(Span<'a>) -> IResult<Span<'a>>,
{
    context(
        "Stripping whitespace",
        delimited(multispace0, inner, multispace0),
    )
}

fn match_marker<'a>(marker: &'a str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>> {
    context("Matching marker", ws(terminated(tag(marker), line_ending)))
}

fn grab_frontmatter<'a>(marker: &'a str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>> {
    context(
        "Grabbing frontmatter",
        recognize(many0_count(tuple((
            not(match_marker(marker)),
            take(1usize),
        )))),
    )
}
