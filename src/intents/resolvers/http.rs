use super::{Intent, Errors, IntentParser};
use std::borrow::Cow::{self, Borrowed};
use hyper::{body::HttpBody as _, Client, Uri};
use url::form_urlencoded::{byte_serialize};
use serde_json::{from_slice};


pub type Value = serde_json::Value;

/// Resolves an input string using a http connection to an LU client
pub struct HttpResolver {
    base_url: String,
    parser: IntentParser
}


impl HttpResolver {
    pub fn new() -> HttpResolver {
        // fetch the base_url from the environment or resolve to a static string defined 
        let base_url = std::env::var("LU_API_URL").unwrap_or("http://luisendpoint.azurewebsites.net/?text=".to_string());

        HttpResolver {
            base_url,
            parser: IntentParser::new()
        }
    }

    fn url_encode(&self, input: &str) -> Result<Uri, Errors> {
        let mut url = self.base_url.clone();
        let param: String = byte_serialize(&input.as_bytes()).collect();

        url.push_str(&param);

        match url.parse::<Uri>(){
            Ok(url) => Ok(url),
            _ => Err(Errors::InvalidInput)
        }
    }

    pub async fn get(&self, input: &str) -> Result<Value, Errors> {
        // create a http url and use a http client to invoke
        let url = self.url_encode(input)?;

        // use hyper to make the request
        let client = Client::new();

        let res = match client.get(url).await {
            Err(err) => return Err(Errors::NetworkError(err.to_string())),
            Ok(res) => res
        };

        let body = match hyper::body::to_bytes(res).await {
            Err(err) => return Err(Errors::NetworkError(err.to_string())),
            Ok(body) => body
        };


        let body: Value = match from_slice(&body){
            Err(_) => return Err(Errors::ParsingError),
            Ok(body) => body
        };

        Ok(body)
    }

}

