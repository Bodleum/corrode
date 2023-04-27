mod frontmatter;
// mod shortcodes;

use nom_locate::LocatedSpan;

pub use frontmatter::*;

use crate::error::ParseError;

// Helper types
pub type Span<'a> = LocatedSpan<&'a str>;
pub type IResult<'a, O, E = ParseError<'a>> = nom::IResult<LocatedSpan<&'a str>, O, E>;
