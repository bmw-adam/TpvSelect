use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::web::spa;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_cors::Cors;

use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use oauth2::reqwest::async_http_client;
use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

#[get("/auth/google")]
async fn google_auth() -> HttpResponse {
    let client = oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .add_scope(Scope::new("email".to_string()))
        .url();

    println!("Redirecting to: {}", auth_url);
    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/auth/google/callback")]
async fn google_callback(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let code = query.get("code").unwrap();
    let client = oauth_client();

    let token_result = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await;

    println!("Token result: {:?}", token_result);

    match token_result {
        Ok(token) => {
            match serde_json::to_string_pretty(&token) {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("Failed to serialize token response: {}", e),
            }

            HttpResponse::Ok().body("Login successful!")
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Auth error: {}", e)),
    }
}

fn oauth_client() -> oauth2::basic::BasicClient {
    let client_id = ClientId::new((&"id").to_string()); //TODO: replace with env var
    let client_secret = ClientSecret::new((&"secret").to_string()); // TODO: replace with env var
    let auth_url = AuthUrl::new("authUrl".to_string()).unwrap(); // TODO: replace with env var
    let token_url = TokenUrl::new("tokenUrl".to_string()).unwrap(); // TODO: replace with env var

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new("https://localhost:1235/auth/google/callback".to_string()).unwrap())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load and validate environment variables at the top
    let key_path = env::var("KEY").expect("KEY environment variable not set");
    let cert_path = env::var("CERT").expect("CERT environment variable not set");
    let dist_path = env::var("DIST").expect("DIST environment variable not set");

    let connection = establish_connection();

    // Set up TLS
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(&key_path, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(&cert_path).unwrap();

    // Start server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(google_auth)
            .service(google_callback)
            .service(
                spa()
                    .index_file(format!("{}/index.html", dist_path.clone()))
                    .static_resources_mount("/")
                    .static_resources_location(dist_path.clone())
                    .finish()
            )
    })
    .bind_openssl("0.0.0.0:1235", builder)?
    .run()
    .await
}
