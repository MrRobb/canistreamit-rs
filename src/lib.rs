
extern crate reqwest;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value, Error};
use reqwest::Method;
use std::fmt::Display;
use std::collections::HashMap;


#[derive(Debug)]
pub enum CanIStreamError {
    RequestError(String),
    NoJson(String)
}

impl std::error::Error for CanIStreamError {}

impl Display for CanIStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CanIStreamError::RequestError(e) => write!(f, "{}", e),
            CanIStreamError::NoJson(e) => write!(f, "{}", e),
        }
    }
}

type Result<T> = std::result::Result<T, CanIStreamError>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TitleType {
    Movie,
    Show
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ScoreType {
    #[serde(alias = "imdb:score")]
    ImdbScore,
    #[serde(alias = "imdb:popularity")]
    ImdbPopularity,
    #[serde(alias = "imdb:votes")]
    ImdbVotes,
    #[serde(alias = "imdb:multiplied")]
    ImdbMultiplied,
    #[serde(alias = "tomato:id")]
    TomatoID,
    #[serde(alias = "tomato:score")]
    TomatoScore,
    #[serde(alias = "tomato:rating")]
    TomatoRating,
    #[serde(alias = "tomato:meter")]
    TomatoMeter,
    #[serde(alias = "tomato_userrating:score")]
    TomatoUserScore,
    #[serde(alias = "tomato_userrating:rating")]
    TomatoUserRating,
    #[serde(alias = "tomato_userrating:meter")]
    TomatoUserMeter,
    #[serde(alias = "tmdb:id")]
    TmdbID,
    #[serde(alias = "tmdb:score")]
    TmdbScore,
    #[serde(alias = "tmdb:popularity")]
    TmdbPopularity,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OfferUrls {
    pub standard_web: String,
    pub deeplink_ios: Option<String>,
    pub deeplink_android: Option<String>,
    pub deeplink_android_tv: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    pub monetization_type: String,
    pub provider_id: u64,
    pub retail_price: Option<f64>,
    pub currency: Option<String>,
    pub urls: OfferUrls,
    pub presentation_type: String,
    pub date_created_provider_id: String,
    pub date_created: String,
    pub country: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    pub provider_type: ScoreType,
    pub value: Number
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub id: u64,
    pub title: String,
    pub full_path: String,
    pub poster: Option<String>,
    pub short_description: Option<String>,
    pub original_release_year: Option<u64>,
    pub tmdb_popularity: f64,
    pub object_type: TitleType,
    pub original_title: String,
    pub localized_release_date: Option<String>,
    pub offers: Option<Vec<Offer>>,
    pub scoring: Vec<Score>,
    pub original_language: Option<String>,
    pub age_certification: Option<String>,
    pub runtime: Option<u64>,
    pub cinema_release_date: Option<String>,
    pub cinema_release_week: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    id: u64,
    clear_name: String,
    short_name: String,
    technical_name: String,
    icon_url: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Params {
    pub content_types: Option<String>,
    pub presentation_types: Option<String>,
    pub providers: Option<String>,
    pub genres: Option<String>,
    pub languages: Option<String>,
    pub release_year_from: Option<String>,
    pub release_year_until: Option<String>,
    pub monetization_types: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub scoring_filter_types: Option<String>,
    pub cinema_release: Option<String>,
    pub query: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

impl Params {
    fn movie_title(mut self, movie: &str) -> Self {
        self.query = Some(movie.into());
        self
    }
}

impl ToString for Params {
    fn to_string(&self) -> String {

        let mut url: String = "".into();

        if let Ok(json) = serde_json::to_string(self) {
            if let Some(j) = serde_json::json!(json).as_object() {
                for (i, (key, val)) in j.iter().enumerate() {
                    if i != 0 {
                        url.push('&');
                    }
                    if let Some(val_str) = val.as_str() {
                        url.push_str(key);
                        url.push('=');
                        url.push_str(val_str);
                    }
                }
            }
        }

        url
    }
}

fn request(method: reqwest::Method, endpoint: String, params: Params) -> Result<Value> {

    // Base url
    let url = format!("https://apis.justwatch.com/content{}", endpoint);

    // Create request
    let response = match &method {
        &reqwest::Method::GET => {

            // Add params to url
            reqwest::Client::new()
                .get((url + "?" + params.to_string().as_str()).as_str())
                .send()
        }
        &reqwest::Method::POST => {

            // Add params to body
            reqwest::Client::new()
                .post(url.as_str())
                .json(&params)
                .send()
        }
        _ => unreachable!()
    };

    match response {
        Ok(mut r) => {
            match r.json() {
                Ok(j) => Ok(j),
                Err(e) => Err(CanIStreamError::NoJson(e.to_string())),
            }
        },
        Err(e) => Err(CanIStreamError::RequestError(e.to_string())),
    }
}

pub fn search(movie: &str) -> Result<Vec<Movie>> {

    let locale = "en_US";
    let url = format!("/titles/{}/popular", locale);

    let params: Params = Params::default()
        .movie_title(movie);

    let j = request(reqwest::Method::POST, url, params)?;
    let mut movies = vec![];

    if let Some(items_json) = j.get("items") {
        if let Some(movies_json) = items_json.as_array() {
            for movie_json in movies_json {
                let movie = serde_json::from_value(movie_json.clone());
                match movie {
                    Ok(m) => {
                        movies.push(m);
                    },
                    Err(e) => {
                        eprintln!("Error adding title: {:?}", e);
                    }
                }
            }
        }
    }

    Ok(movies)
}

pub fn get_providers() -> Result<HashMap<u64, Provider>> {
    let locale = "en_US";
    let url = format!("/providers/locale/{}", locale);

    let j = request(Method::GET, url, Params::default())?;

    let mut providers = HashMap::new();

    if let Some(arr_json) = j.as_array() {
        for prov_json in arr_json {
            let provider: std::result::Result<Provider, Error> = serde_json::from_value(prov_json.clone());
            match provider {
                Ok(p) => { providers.insert(p.id, p); },
                Err(e) => eprintln!("Error processing provider: {:?}", e.to_string()),
            }
        }
    }

    Ok(providers)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::error::Error;


    #[test]
    fn test_search() {

        let movies = search("Interstellar");

        match movies {
            Ok(m) => assert_ne!(0, m.len()),
            Err(e) => {
                eprintln!("Error: {}", e.description());
                assert!(false);
            },
        }
    }

    #[test]
    fn test_providers() {

        let providers = get_providers();
        
        match providers {
            Ok(p) => assert!(p.len() >= 104),
            Err(e) => {
                eprintln!("Error: {}", e.description());
                assert!(false);
            },
        }
    }
}
