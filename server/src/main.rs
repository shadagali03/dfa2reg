//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

use serde::Deserialize;

use dfa2reg::scanner::Scanner;
//use std::{env, process};
use std::process;

#[cfg(test)]
mod tests;

#[derive(Deserialize, Debug)]
struct UserSystem {
    alphabet: String,
    states: String,
    initial: String,
    accepting: String,
    transitions: String,
}

// #[get("/generate_regex")]
// async fn generate_regex() -> impl Responder {
//     let scanner = Scanner::new();
//     let path = String::from("src/inputs/ex_input2.txt");
//     let res: String;
//     match scanner.run_file(path) {
//         Ok(regex) => {
//             println!("Regex: {regex}");
//             res = regex;
//             // process::exit(0);
//         }
//         Err(msg) => {
//             eprintln!("{}", msg);
//             process::exit(1);
//         }
//     }
//     HttpResponse::Ok().body(res)
// }

async fn handle_regex(data: web::Json<UserSystem>) -> Result<String> {
    println!("HERE");
    Ok(format!("Data recieved: {}", data.accepting))
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
    HttpServer::new(|| App::new().route("/regex", web::post().to(handle_regex)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
