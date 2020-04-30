use super::{Errors, LuResponse};
use hyper::{Client, Uri};
use url::form_urlencoded::{byte_serialize};
use serde_json::{from_slice};

/// Resolves an input string using a http connection to an LU client
pub struct HttpResolver {
    base_url: String,
}


impl HttpResolver {
    pub fn new() -> HttpResolver {
        // fetch the base_url from the environment or resolve to a static string defined 
        let base_url = std::env::var("LU_API_URL").unwrap_or("http://luisendpoint.azurewebsites.net/?text=".to_string());

        HttpResolver {
            base_url,
        }
    }

    fn url_encode(&self, input: &str) -> Result<Uri, Errors> {
        let mut url = self.base_url.clone();
        let param: String = byte_serialize(&input.as_bytes()).collect();

        url.push_str(&param);

        match url.parse::<Uri>(){
            Ok(url) => Ok(url),
            _ => Err(Errors::InvalidInput(input.to_string()))
        }
    }

    pub async fn get(&self, input: &str) -> Result<LuResponse, Errors> {
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


        let body: LuResponse = match from_slice(&body){
            Err(err) => {
                println!("{:?}", err);
                return Err(Errors::ParsingError);
            },
            Ok(body) => body
        };

        Ok(body)
    }

}

