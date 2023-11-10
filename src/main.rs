// main.rs
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use rand::Rng;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Deserialize, Serialize)]
struct InputData {
    input: String,
}


lazy_static! {
    static ref DICT: HashSet<String> = load_dict_from_file("data/words_beta.txt").unwrap();
	static ref TARGET_WORD: Mutex<String> = Mutex::new(String::from(""));
}

#[tokio::main]
async fn main() {
    if let Err(e) = real_main().await {
        eprintln!("Error: {}", e);
    }
}

async fn real_main() -> Result<(), std::io::Error> {
    let current_dir = std::env::current_dir()?;
    let homepage_path = current_dir.join("frontend/index.html");
    let homepage = warp::path("index.html")
        .and(warp::fs::file(homepage_path));
    let script_path = current_dir.join("frontend/script.js");
    let script = warp::path("script.js")
        .and(warp::fs::file(script_path));
    let process_input_filter = warp::path("process_input")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(process_input);
    let get_new_word_filter = warp::path("get_new_word")
        .and(warp::get())
        .map(|| {
            get_new_word();
        	let target_word = TARGET_WORD.lock().unwrap().clone();
        	warp::reply::json(&target_word)
        });
	let routes = homepage.or(script).or(process_input_filter).or(get_new_word_filter);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

async fn process_input(input: InputData) -> Result<impl Reply, Rejection> {
    let word_guess = input.input.to_lowercase();
    if word_guess.len() != 5 {
        let error_response = warp::reply::with_status(warp::reply::json(&"Enter a 5-letter word"), warp::http::StatusCode::PARTIAL_CONTENT);
        return Ok(error_response);
    }
	if !DICT.contains(&word_guess) {
	    let error_response = warp::reply::with_status(warp::reply::json(&"Not a valid word"), warp::http::StatusCode::PARTIAL_CONTENT);
        return Ok(error_response);
	}
	let guess_mask = create_guess_mask(&word_guess);
    let processed_word = word_guess.to_uppercase();
    let target_word = TARGET_WORD.lock().unwrap();
    let json_data = json!({
    "processed_word": processed_word,
    "guess_mask": guess_mask,
    "target_word": &*target_word,
    });
    let success_response = warp::reply::with_status(warp::reply::json(&json_data), warp::http::StatusCode::OK);

    Ok(success_response)
}

fn load_dict_from_file(file_path: &str) -> Result<HashSet<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut dictionary = HashSet::new();

    for line in reader.lines() {
        if let Ok(word) = line {
            dictionary.insert(word.trim().to_lowercase());
        }
    }

    Ok(dictionary)
}

fn get_new_word() {
    let dict_vec: Vec<&'static str> = DICT.iter().map(|s| s.as_str()).collect();

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..dict_vec.len());
    
    let mut target_word = TARGET_WORD.lock().unwrap();
    target_word.clear();
    target_word.push_str(dict_vec[index]);
}

fn create_guess_mask(word_guess: &str) -> Vec<String> {
	let mut guess_mask: Vec<String> = Vec::new();
	let target_word = TARGET_WORD.lock().unwrap();

    for (index, letter) in word_guess.chars().enumerate() {
        let mut colour = "Gray".to_string();
        if letter == target_word.chars().nth(index).unwrap() {
            colour = "Green".to_string();
        } else if target_word.contains(letter) {
            colour = "Yellow".to_string();
        }
        guess_mask.push(colour);
    }
   	guess_mask
}