use serde::{Deserialize, Serialize};
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use thiserror::Error;
use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    pub static ref HELP: String = {
        help::HelpMessage::new()
            .name("joke")
            .description("Request a bad joke from the internet")
            .add_subcommand(
                help::HelpMessage::new()
                    .name("random")
                    .description("Request a random joke")
                    .clone(),
            )
            .to_string()
    };
}

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum JokeError
{
    #[error("RequestError: {message}")]
    Request {message: String},

    #[error("JSONParseError: {message}")]
    JsonParse {message: String},
}


pub fn random_joke() -> Result<String, JokeError>
{
    type Error = JokeError;
    // Request URL
    let url = "https://v2.jokeapi.dev/joke/Any?blacklistFlags=racist,sexist".to_string();
    let mut joke = String::new();
    // Get the response
    if let Ok(resp) = match reqwest::blocking::get(url) {
        Ok(x) => x,
        Err(e) => {
            return Err(Error::Request {
                message: format!("{e}"),
            })
        }
    }
    .json::<Value>()
    {
        let joke_kind = match resp.get("type")
        {
            Some(x) => x.as_str().unwrap(),
            None => return Err(Error::JsonParse {message: "Invalid JSON content".to_string()})
        };

        match joke_kind
        {
            "twopart" => {
                if let Some(setup) = resp.get("setup")
                {
                    if let Some(delivery) = resp.get("delivery")
                    {
                        joke = format!("{}\n||{}||", setup.as_str().unwrap(), delivery.as_str().unwrap());
                    }
                }
            }
            "single" => {
                if let Some(bad_joke) = resp.get("joke")
                {
                    joke = bad_joke.as_str().unwrap().to_string();
                }
            }
            
            _=> unreachable!()
        }
    } else {
        return Err(Error::JsonParse {
            message: "Invalid JSON content".to_string(),
        });
    }

    if joke.is_empty()
    {
       Err(Error::JsonParse {message: "Invalid JSON content".to_string()}) 
    }
    else
    {
        Ok(joke)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_random_joke_doesnt_panic()
    {
        random_joke().unwrap();
    }
}

