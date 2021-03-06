#![deny(missing_docs)]

//! numnums
//!
//! ---
//!
//! reusable parsers to feed your nom
//!
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::Parser;

/// finds either \(\) or \(something\)
pub struct Parens;

impl<'a> Parser<&'a str, &'a str, ()> for Parens {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        alt((NonEmptyParensPair, EmptyParensPair))(input)
    }
}

/// finds "\("
pub struct LeftParens;

impl<'a> Parser<&'a str, &'a str, ()> for LeftParens {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        tag("(")(input)
    }
}

/// finds "\)"
pub struct RightParens;

impl<'a> Parser<&'a str, &'a str, ()> for RightParens {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        tag(")")(input)
    }
}

/// finds "\(something\)"
pub struct NonEmptyParensPair;

impl<'a> Parser<&'a str, &'a str, ()> for NonEmptyParensPair {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        delimited(LeftParens, is_not(")"), RightParens)(input)
    }
}

/// finds "\(\)"
pub struct EmptyParensPair;

impl<'a> Parser<&'a str, &'a str, ()> for EmptyParensPair {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        terminated(LeftParens, RightParens)(input)
    }
}

/// finds "!\["
pub struct LeftMarkdownImageBracket;

impl<'a> Parser<&'a str, &'a str, ()> for LeftMarkdownImageBracket {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        tag("![")(input)
    }
}

/// finds "!\[\]"
pub struct EmptyMarkdownImageBracketPair;

impl<'a> Parser<&'a str, &'a str, ()> for EmptyMarkdownImageBracketPair {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        terminated(LeftMarkdownImageBracket, RightBracket)(input)
    }
}

/// finds "!\[something\]"
pub struct NonEmptyMarkdownImageBracketPair;

impl<'a> Parser<&'a str, &'a str, ()> for NonEmptyMarkdownImageBracketPair {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        delimited(LeftMarkdownImageBracket, is_not("]"), RightBracket)(input)
    }
}

/// finds "\["
pub struct LeftBracket;

impl<'a> Parser<&'a str, &'a str, ()> for LeftBracket {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        tag("[")(input)
    }
}

/// finds "\]"
pub struct RightBracket;

impl<'a> Parser<&'a str, &'a str, ()> for RightBracket {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        tag("]")(input)
    }
}

/// finds "\[something\]"
pub struct NonEmptyBracketPair;

impl<'a> Parser<&'a str, &'a str, ()> for NonEmptyBracketPair {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        delimited(LeftBracket, is_not("]"), RightBracket)(input)
    }
}

/// finds "\[\]"
pub struct EmptyBracketPair;

impl<'a> Parser<&'a str, &'a str, ()> for EmptyBracketPair {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        terminated(LeftBracket, RightBracket)(input)
    }
}

/// finds either "\[\]" or "\[something\]"
pub struct Brackets;

impl<'a> Parser<&'a str, &'a str, ()> for Brackets {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        alt((NonEmptyBracketPair, EmptyBracketPair))(input)
    }
}

/// finds either "!\[\]" or "!\[something\]"
pub struct MarkdownImageBrackets;

impl<'a> Parser<&'a str, &'a str, ()> for MarkdownImageBrackets {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, &'a str), nom::Err<()>> {
        alt((
            NonEmptyMarkdownImageBracketPair,
            EmptyMarkdownImageBracketPair,
        ))(input)
    }
}

/// finds all urls "\[maybe_something\]\(maybe_something\)"
/// but not image urls "\![maybe_something\]\(maybe_something\)"
pub struct MarkdownUrls;

/// type alias for complex type used as result from MarkdownUrls
pub type MarkdownUrlsResults<'a> =
    Result<(&'a str, Vec<(&'a str, (&'a str, &'a str))>), nom::Err<()>>;

impl<'a> Parser<&'a str, Vec<(&'a str, (&'a str, &'a str))>, ()> for MarkdownUrls {
    fn parse(&mut self, input: &'a str) -> MarkdownUrlsResults<'a> {
        fold_many0(
            pair(take_until("["), MarkdownUrl),
            Vec::new,
            |mut acc: Vec<_>, item| {
                //here we want to inspect what we took_until with `take_until` so we can verify
                //we want to actually accumulate this instead of skipping it
                //but how in the heck to we get access to it each time we fold?
                if !item.0.ends_with('!') {
                    acc.push(item);
                }
                acc
            },
        )(input)
    }
}

/// finds "\[maybe_something\]\(maybe_something\)"
pub struct MarkdownUrl;

impl<'a> Parser<&'a str, (&'a str, &'a str), ()> for MarkdownUrl {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, (&'a str, &'a str)), nom::Err<()>> {
        tuple((Brackets, Parens))(input)
    }
}

/// finds all images "!\[maybe_something\]\(maybe_something\)"
/// but not plain markdown urls "\[maybe_something\]\(maybe_something\)"
pub struct MarkdownImages;

/// type alias for complex type used as result from MarkdownImages
pub type MarkdownImagesResults<'a> = Result<(&'a str, Vec<(&'a str, &'a str)>), nom::Err<()>>;

impl<'a> Parser<&'a str, Vec<(&'a str, &'a str)>, ()> for MarkdownImages {
    fn parse(&mut self, input: &'a str) -> MarkdownImagesResults<'a> {
        many0(preceded(
            take_until("!["),
            tuple((MarkdownImageBrackets, Parens)),
        ))(input)
    }
}

/// finds "!\[maybe_something\]\(maybe_something\)"
pub struct MarkdownImage;

impl<'a> Parser<&'a str, (&'a str, &'a str), ()> for MarkdownImage {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, (&'a str, &'a str)), nom::Err<()>> {
        tuple((MarkdownImageBrackets, Parens))(input)
    }
}

/// finds a vec of words \(including punctuation\) created from the alt text of a markdown image: "!\[alt text\]\(https://some_url\)"
pub struct MarkdownImageAltText;

impl<'a> Parser<&'a str, Vec<&'a str>, ()> for MarkdownImageAltText {
    fn parse(&mut self, input: &'a str) -> Result<(&'a str, Vec<&'a str>), nom::Err<()>> {
        MarkdownImage.parse(input).map(|v| {
            let image = v.1;
            // now we need to parse the words which could be separated by one ore more spaces
            // like so: word1 word2   word3  word4
            let words = image.0.split_ascii_whitespace().collect();
            (image.1, words)
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use nom::{
        bytes::complete::is_not,
        combinator::recognize,
        sequence::{delimited, terminated},
    };

    #[test]
    fn recognize_markdown_image_alt_text_sentence() -> anyhow::Result<()> {
        let input = "![I am a great description.  Thanks for reading me!](https://www.google.com)";

        let token = MarkdownImageAltText.parse(input)?;

        assert_eq!(token.0, "https://www.google.com");

        assert_eq!(
            token.1,
            vec![
                "I",
                "am",
                "a",
                "great",
                "description.",
                "Thanks",
                "for",
                "reading",
                "me!"
            ]
        );

        assert_eq!(token.1.len(), 9);
        Ok(())
    }

    #[test]
    fn recognize_markdown_image_alt_text_words() -> anyhow::Result<()> {
        let input = "![word word  word](some_url)";

        let token = MarkdownImageAltText.parse(input)?;

        assert_eq!(token.1, vec!["word", "word", "word"]);

        Ok(())
    }

    #[test]
    fn recognize_markdown_image() -> anyhow::Result<()> {
        let input = "![image](abcd)";

        let token = recognize(MarkdownImage)(input)?;

        assert_eq!(token, ("", "![image](abcd)"));

        let input = "![image word](abcd)";

        let token = recognize(MarkdownImage)(input)?;

        assert_eq!(token, ("", "![image word](abcd)"));

        Ok(())
    }

    #[test]
    fn recognize_markdown_url_not_image() -> anyhow::Result<()> {
        let input = "here's some other stuff ![image](abcd) here's [some anchor](please find me!) some more stuff";

        let token = MarkdownUrls.parse(input)?;
        let count = token.1.len();

        assert_eq!(count, 1);
        //assert_eq!(token.1, vec![("", "[image](abcd)")]);

        Ok(())
    }

    #[test]
    fn recognize_markdown_url() -> anyhow::Result<()> {
        let input = "[image](abcd)";

        let token = recognize(MarkdownUrl)(input)?;

        assert_eq!(token, ("", "[image](abcd)"));

        Ok(())
    }

    #[test]
    fn recognize_non_empty_markdown_image_brackets() -> anyhow::Result<()> {
        let input = "![abcd]";

        let token = recognize(NonEmptyMarkdownImageBracketPair)(input)?;

        assert_eq!(token, ("", "![abcd]"));

        Ok(())
    }

    #[test]
    fn recognize_non_empty_parens() -> anyhow::Result<()> {
        let input = "(abcd)";

        let token = recognize(NonEmptyParensPair)(input)?;

        assert_eq!(token, ("", "(abcd)"));

        Ok(())
    }

    #[test]
    fn recognize_empty_value() -> anyhow::Result<()> {
        let input = "[]asdf[]";

        let token = recognize(EmptyBracketPair)(input)?;

        assert_eq!(token, ("asdf[]", "[]"));

        Ok(())
    }

    #[test]
    fn empty_value() -> anyhow::Result<()> {
        let input = "[]";

        let token = terminated(LeftBracket, RightBracket)(input)?;

        assert_eq!(token, ("", "["));

        Ok(())
    }

    #[test]
    fn bracketed_value() -> anyhow::Result<()> {
        let input = "[abc]";

        let token = delimited(LeftBracket, is_not("]"), RightBracket)(input)?;

        assert_eq!(token, ("", "abc"));

        Ok(())
    }
}
