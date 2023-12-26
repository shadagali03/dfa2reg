use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};

use serde::Deserialize;

use dfa2reg::gnfa_process;
use dfa2reg::scanner::Scanner;
use std::process;
// use std::{env, process};

#[cfg(test)]
mod tests;

#[derive(Deserialize, Debug, Clone)]
struct UserSystem {
    alphabet: String,
    states: String,
    initial: String,
    accepting: String,
    transitions: String,
}

#[derive(Debug, serde::Serialize)]
struct Regex {
    regex: String,
}

#[get("/generate_regex")]
async fn generate_regex() -> impl Responder {
    let scanner = Scanner::new();
    let path = String::from("src/inputs/ex_input2.txt");
    let res: Regex;
    match scanner.run_file(path) {
        Ok(regex) => {
            println!("Regex: {regex}");
            res = Regex { regex: regex };
            // process::exit(0);
        }
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    }
    HttpResponse::Ok().json(res)
}

async fn handle_regex(data: web::Json<UserSystem>) -> impl Responder {
    let mut user_input = vec![
        data.alphabet.clone(),
        data.states.clone(),
        data.initial.clone(),
        data.accepting.clone(),
    ];
    let res: Regex;

    user_input.extend(
        data.transitions
            .clone()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );

    println!("{:?}", user_input);
    let mut value: String = "".to_string();
    match Scanner::parse_input(user_input) {
        Ok(mut transition_table) => {
            value = gnfa_process::run_gnfa(&mut transition_table).unwrap();
        }
        Err(_) => (),
    }
    res = Regex { regex: value };
    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // let scanner = Scanner::new();
    // let mut path = String::from("src/inputs/ex_input2.txt");

    // if args.len() > 2 {
    //     eprintln!("Too many arguments");
    //     process::exit(64);
    // } else if args.len() == 2 {
    //     path = format!("src/inputs/{}", args.get(1).unwrap());
    // }
    // match scanner.run_file(path) {
    //     Ok(regex) => {
    //         println!("Regex: {regex}");
    //         // process::exit(0);
    //     }
    //     Err(msg) => {
    //         eprintln!("{}", msg);
    //         process::exit(1);
    //     }
    // }
    println!("running on port 127.0.0.1");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(generate_regex)
            .route("/regex", web::post().to(handle_regex))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
