use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

pub const HELP: &str =
r#"quote: Request quotes from the internet.
Subcommands
    random: Request a random quote."#;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("quote")
        .dm_permission(true)
        .description("Request quotes from the internet")
        .create_option(|option| {
            option
                .name("random")
                .kind(CommandOptionType::SubCommand)
                .description("Request a random quote")
        })
}

#[derive(Error,Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuoteError
{
    #[error("JSONParseError: couldn't parse json: {message}")]
    JsonParse {message: String},

    #[error("RequestError: couldn't request quote: {message}")]
    Request {message: String},

    #[default]
    #[error("Error: An unknown error occurred.")]
    GeneralError,
}

pub enum Quote
{
    Random(RandomQuote),
}

impl Quote
{
    pub fn get_random() -> Result<Self, QuoteError>
    {
        Ok(Self::Random(RandomQuote::fetch()?))
    }
}

impl std::fmt::Display for Quote
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", match self {Self::Random(quote) => quote})
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomQuote {
    #[serde(rename = "_id")]
    pub id: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<String>,
    pub author_slug: String,
    pub length: i64,
    pub date_added: String,
    pub date_modified: String,
}

impl RandomQuote
{
    pub fn fetch() -> Result<Self, QuoteError>
    {
        // Construct request URL
        let url = "https://api.quotable.io/random".to_string();

        // Get the response
        if let Ok(resp) = match reqwest::blocking::get(url)
        {
            Ok(x) => x,
            Err(e) =>
            {
                return Err(QuoteError::Request {
                    message: format!("{e}"),
                })
            }
        }
        .json::<Self>()
        {
            Ok(resp)
        }
        else
        {
            Err(QuoteError::JsonParse {
                message: "Invalid JSON content".to_string(),
            })
        }
    }
}

impl std::fmt::Display for RandomQuote
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "*{}*\n\t— {}", self.content.trim(), self.author)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    
    #[test]
    fn test_random_quote_display()
    {
        let quote = RandomQuote {
            id: "NONE".to_string(),
            content: "This is a quote".to_string(),
            author: "Rust Test".to_string(),
            tags: Vec::new(),
            author_slug: "NONE".to_string(),
            length: 17,
            date_added: "NONE".to_string(),
            date_modified: "NONE".to_string(),
        };
        
        assert_eq!(quote.to_string(), "*This is a quote*\n\t— Rust Test");
    }
    
    #[test]
    fn test_quote_display()
    {
        let quote = Quote::Random(RandomQuote {
            id: "NONE".to_string(),
            content: "This is a quote".to_string(),
            author: "Rust Test".to_string(),
            tags: Vec::new(),
            author_slug: "NONE".to_string(),
            length: 17,
            date_added: "NONE".to_string(),
            date_modified: "NONE".to_string(),
        });
        
        assert_eq!(quote.to_string(), "*This is a quote*\n\t— Rust Test");
    }
}
