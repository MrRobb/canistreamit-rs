
extern crate reqwest;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Number;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum TitleType {
    Movie,
    Show
}

#[derive(Serialize, Deserialize, Debug)]
enum ScoreType {
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
struct OfferUrls {
    standard_web: String,
    deeplink_ios: Option<String>,
    deeplink_android: Option<String>,
    deeplink_android_tv: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Offer {
    monetization_type: String,
    provider_id: u64,
    retail_price: Option<f64>,
    currency: Option<String>,
    urls: OfferUrls,
    presentation_type: String,
    date_created_provider_id: String,
    date_created: String,
    country: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Score {
    provider_type: ScoreType,
    value: Number
}

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: u64,
    title: String,
    full_path: String,
    poster: Option<String>,
    short_description: Option<String>,
    original_release_year: Option<u64>,
    tmdb_popularity: f64,
    object_type: TitleType,
    original_title: String,
    localized_release_date: Option<String>,
    offers: Option<Vec<Offer>>,
    scoring: Vec<Score>,
    original_language: Option<String>,
    age_certification: Option<String>,
    runtime: Option<u64>,
    cinema_release_date: Option<String>,
    cinema_release_week: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Params {
    content_types: Option<String>,
    presentation_types: Option<String>,
    providers: Option<String>,
    genres: Option<String>,
    languages: Option<String>,
    release_year_from: Option<String>,
    release_year_until: Option<String>,
    monetization_types: Option<String>,
    min_price: Option<String>,
    max_price: Option<String>,
    scoring_filter_types: Option<String>,
    cinema_release: Option<String>,
    query: Option<String>,
    page: Option<String>,
    page_size: Option<String>,
}

impl Params {

    fn default() -> Self {
        Self {
            content_types: None,
            presentation_types: None,
            providers: None,
            genres: None,
            languages: None,
            release_year_from: None,
            release_year_until: None,
            monetization_types: None,
            min_price: None,
            max_price: None,
            scoring_filter_types: None,
            cinema_release: None,
            query: None,
            page: None,
            page_size: None
        }
    }

    fn movie_title(mut self, movie: &str) -> Self {
        self.query = Some(movie.into());
        self
    }

}

impl ToString for Params {
    fn to_string(&self) -> String {
        let mut url: String = "".into();
        let json: serde_json::Value = serde_json::json!(serde_json::to_string(self).unwrap());
        if json.is_object() {
            for (i, (key, val)) in json.as_object().unwrap().iter().enumerate() {
                if i != 0 {
                    url.push('&');
                }
                url.push_str(key);
                url.push('=');
                url.push_str(val.as_str().unwrap());
            }
        }
        url
    }
}

fn request(method: reqwest::Method, endpoint: String, params: Params) -> Vec<Movie> {

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

            println!("{}", serde_json::to_string(&params).unwrap());

            // Add params to body
            reqwest::Client::new()
                .post(url.as_str())
                .json(&params)
                .send()
        }
        _ => unreachable!()
    };

    let mut movies = vec![];
    let j: serde_json::Value = response.unwrap().json().unwrap();

    println!("{}", j);

    for movies_json in j.get("items") {
        for movie_json in movies_json.as_array().unwrap() {
            let movie = serde_json::from_value(movie_json.clone());
            if movie.is_ok() {
                movies.push(movie.unwrap());
            }
            else {
                println!("error = {:?}", movie.err().unwrap());
            }
        }
    }

    movies
}

fn search(movie: &str) -> Vec<Movie> {

    let locale = "en_US";
    let url = format!("/titles/{}/popular", locale);

    let params: Params = Params::default()
        .movie_title(movie);

    request(reqwest::Method::POST, url, params)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search_movie() {
        let movies = search("Interstellar");
        assert_ne!(0, movies.len());
    }
}
