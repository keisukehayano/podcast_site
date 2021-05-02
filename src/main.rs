use actix_multipart::Multipart;
use actix_web::{ HttpRequest, get, guard, Result, error, middleware, web, App, Error, HttpResponse, HttpServer, Responder };
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};

use crate::mysql_connection::db_connection;
use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::insert_into;
use diesel::types::Text;
use diesel::types::Integer;
use actix_web::http::{ header, Method, StatusCode };
use actix_files as fs;



use tera::{ Tera, Context };


extern crate tera;

mod mysql_connection;
//mod errors;


type DB = diesel::mysql::Mysql;


#[derive(Debug)]
pub struct Item {
    item_id: i32,
    name: String,
    listen_count: i32,
    datecreate: String,
}

impl QueryableByName<DB> for Item {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Item {
            item_id: row.get("item_id")?,
            name: row.get("name")?,
            listen_count: row.get("listen_count")?,
            datecreate: row.get("datecreate")?,
        })
    }
}

async fn test_sql() -> Result<HttpResponse, Error> {
    let connection: MysqlConnection = db_connection();
    let items: Vec<Item> = sql_query("SELECT item_id, name, listen_count, datecreate FROM items ORDER BY datecreate DESC",).load(&connection).unwrap();

    println!("{:?}", items);
    Ok(HttpResponse::Ok().into())
}

// itemデータ挿入 
fn item_data_access_insert() {
    let connection: MysqlConnection = db_connection();
    let query = sql_query("INSERT INTO items(item_id, name, listen_count, datecreate)VALUES(?, ?, ?, ?)")
        .bind::<Integer, _>(3)
        .bind::<Text, _>("test4")
        .bind::<Integer, _>(100)
        .bind::<Text, _>("20210424")
        .execute(&connection);

        println!("{:?}",query);
}

// データ挿入テスト
async fn insert_test() -> Result<HttpResponse, Error> {
    item_data_access_insert();

    Ok(HttpResponse::Ok().into())
}

// LoginForm
async fn login_form(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    let view = tmpl
        .render("login.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))


} 

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form action="/regist/save_file" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn search() -> HttpResponse {
    let html = r#"<html>
        <head><title>Serach Test</title></head>
        <body>
            <form action="/serach/serach_test" method="get" enctype="multipart/form-data">
                <input type="text" multiple name="text"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}


/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/errors/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}


/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// 404.css handler
#[get("/404.css")]
async fn c404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/errors/404.css")?)
}

/// 404.js handler
#[get("/404.js")]
async fn j404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/errors/404.js")?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    async_std::fs::create_dir_all("./tmp").await?;

    let templates = Tera::new("templates/**/*").unwrap();

    let ip = "127.0.0.1:8080";

    HttpServer::new(move || {
        App::new().data(templates.clone()).wrap(middleware::Logger::default())
        .service(
            web::scope("/serach")
                .route("/serach.html", web::get().to(search))
                .route("/serach_test", web::get().to(test_sql)),
        )
        .service(
            web::scope("/regist")
            .route("/index.html", web::get().to(index))
            .route("/save_file", web::post().to(save_file))
            .route("/test_insert", web::post().to(insert_test)),   
        )
        .service(
            web::scope("/login")
            .route("/login_form", web::get().to(login_form)),
        )
        // favicon
        .service(favicon)
        // 404.css
        .service(c404)
        // 404.js
        .service(j404)
        // default
        .default_service(
            // 404 for GET request
            web::resource("")
                .route(web::get().to(p404))
                // all requests that are not `GET`
                .route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
        )
              
    })
    .bind(ip)
    .expect("Can not bind to port 8080")
    .run()
    .await
}