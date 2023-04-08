#[macro_use]
extern crate rocket;
use rocket::{serde::{Serialize, Deserialize, json::Json}, Response};

use rusqlite::{Connection, Result};

use rand::distributions::{Alphanumeric, DistString};

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RequestBody{
    url_to_shorten: String,
    custom_link: Option<String>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResponseBody{
    shortened_url: Option<String>,
    error: Option<String>
}

static HOST_URI: &str = "http://127.0.0.1:8001";

#[post("/shorten",data = "<request_body>")]
fn shorten_url_handler(request_body: Json<RequestBody>) -> Json<ResponseBody> {

    let db = match Connection::open("urls.db"){
        Ok(c) => c,
        Err(e) => {
            println!("1");
            eprintln!("{}", e.to_string());
            return Json(ResponseBody {shortened_url: None, error: Some(e.to_string())})
        }
    };

    match db.execute("CREATE TABLE IF NOT EXISTS urls (
        id TEXT(7) PRIMARY KEY,
        fullUrl TEXT(512) NOT NULL,
        time INTEGER NOT NULL
    )",()) {
        Ok(result) =>{
            println!("{}", result)
        },
        Err(err)=>{
            println!("2");
            eprintln!("{}",err.to_string());
            return Json(ResponseBody {shortened_url: None, error: Some(err.to_string())})
        }
    }

    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 7);
    println!("{}", string.to_lowercase());

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

    match db.execute("INSERT INTO urls (id,fullUrl,time) VALUES (?1, ?2, ?3)", (&string, &request_body.url_to_shorten, timestamp)){
        Ok(result) => {
            if (result == 1) {
                println!("Data Inserted");
                println!("{}",result);

                return Json(ResponseBody { shortened_url: Some(format!("{}/{}",HOST_URI, string)), error: None });

            }
        },
        Err(err) => {
            println!("Insertion Failed");
            eprintln!("{}", err.to_string());

            return Json(ResponseBody{shortened_url: None, error: Some(err.to_string())});
        }
    };


    Json(ResponseBody { shortened_url: Some(request_body.url_to_shorten.to_string()), error: None })
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/v1", routes![shorten_url_handler])
}