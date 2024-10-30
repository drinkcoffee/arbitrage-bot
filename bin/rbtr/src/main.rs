use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    url_one: String,
    url_two: String,
    token_one: String,
    token_two: String,
}

fn main() {}
