use crate::structs::{Message, MojangProfile, Premium};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use once_cell::sync::Lazy;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;
use tokio_postgres::{Error, NoTls};

#[allow(dead_code)]
static CACHE: Lazy<Mutex<HashMap<String, (bool, i32, i32)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Deserialize)]
struct UuidBody {
    uuid: Option<String>,
    username: Option<String>,
}

async fn get_premium(uuid: &str) -> Result<(bool, i32, i32), Error> {
    let mut cache = match CACHE.lock() {
        Ok(lock) => lock,
        Err(poisoned) => poisoned.into_inner(),
    };

    if let Some(&(active, starts_at, ends_at)) = cache.get(uuid) {
        return Ok((active, starts_at, ends_at));
    }

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| String::from("default_database_url"));

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("../../logs/connection_errors.log")
                .unwrap();

            if let Err(_e) = writeln!(file, "connection error: {}", e) {
                eprintln!("Sorry we ran into an error has occurred, please report this to TeaClient Contribitors.");
            }
        }
    });

    let row = client
        .query_one(
            "SELECT active, starts_at, ends_at FROM Premium WHERE uuid = $1 OR username = $1",
            &[&uuid],
        )
        .await;

    let result = match row {
        Ok(row) => {
            let active: bool = row.get(0);
            let starts_at: i32 = row.get::<_, i64>(1) as i32;
            let ends_at: i32 = row.get::<_, i64>(2) as i32;
            (active, starts_at, ends_at)
        }
        Err(_) => (false, 0, 0),
    };

    cache.insert(uuid.to_string(), result);

    Ok(result)
}

async fn username_to_uuid(username: &str) -> String {
    let url = format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        username
    );
    let res = reqwest::get(&url).await;
    match res {
        Ok(response) => {
            let json: Result<MojangProfile, _> = response.json().await;
            match json {
                Ok(data) => data.id,
                Err(_) => "none".to_string(),
            }
        }
        Err(_) => "none".to_string(),
    }
}

async fn uuid_to_username(uuid: &str) -> String {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );
    let res = reqwest::get(&url).await;
    match res {
        Ok(response) => {
            let json: Result<MojangProfile, _> = response.json().await;
            match json {
                Ok(data) => data.name,
                Err(_) => "none".to_string(),
            }
        }
        Err(_) => "none".to_string(),
    }
}

#[post("/premium")]
async fn tea_plus(req: HttpRequest, uuid_body: web::Json<UuidBody>) -> impl Responder {
    let res = if let Some(uuid) = &uuid_body.uuid {
        uuid.clone()
    } else if let Some(username) = &uuid_body.username {
        username_to_uuid(&username).await
    } else {
        return HttpResponse::BadRequest().json(Message {
            message: "Both uuid and username cannot be empty",
        });
    };

    let headers = req.headers();
    let get_header = |name| {
        headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("none")
    };

    let content_type = get_header("Content-Type");
    let accept = get_header("Accept");

    if content_type != "application/json" || accept != "application/json" {
        return HttpResponse::BadRequest().json(Message {
            message: "Invalid Content-Type or Accept header",
        });
    }

    match res.as_str() {
        "none" => HttpResponse::InternalServerError().json(Message {
            message: "Request failed",
        }),
        _ => {
            let (active, starts_at, ends_at) = get_premium(&res).await.unwrap_or((false, 0, 0));
            HttpResponse::Ok().json(Premium {
                uuid: res.clone(),
                name: uuid_to_username(&res).await,
                active,
                ends: ends_at as i64,
                starts: starts_at as i64,
                success: true,
            })
        }
    }
}
