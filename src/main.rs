use actix_multipart::Multipart;
//use actix_web::{ HttpRequest, get, post, guard, Result, error, middleware, web, App, Error, HttpResponse, HttpServer, Responder };
use actix_web::{ get, post, guard, Result, error, middleware, web, App, Error, HttpResponse, HttpServer };
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};

use crate::mysql_connection::db_connection;
use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_query;
//use diesel::insert_into;
use diesel::sql_types::Text;
use diesel::sql_types::Integer;
//use actix_web::http::{ header, Method, StatusCode };
use actix_web::http::{ StatusCode };
use actix_files as fs;
use actix_session::{ CookieSession, Session };
use serde::{ Deserialize, Serialize };
use pwhash::bcrypt;
//use uuid::Uuid;
//use chrono::{Utc, Local, DateTime, Date};
use chrono::{ Utc };


use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use tera::{ Tera, Context };


extern crate tera;

mod mysql_connection;
//mod errors;


type DB = diesel::mysql::Mysql;

// items Data
#[derive(Deserialize, Serialize, Debug,  Clone)]
pub struct Item {
    item_id: i32,
    name: String,
    file_pass: String,
    listen_count: i32,
    datecreate: String,
}

// Item And ShowNote
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemAndShowNote {
    item_id: i32,
    name: String,
    file_pass: String,
    listen_count: i32,
    datecreate: String,
    note: Option<String>,
    note_long: Option<String>,
}


// LoginFormData
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Form {
    login_id: String,
    user_pass: String,
}

// shownote Data
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShowNote {
    item_id: i32,
    note: Option<String>,
    note_long: Option<String>,
}


// Form(UserInfo)
impl QueryableByName<DB> for Form {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Form {
            login_id: row.get("login_id")?,
            user_pass: row.get("user_pass")?,
        })
    }
}

// Item
impl QueryableByName<DB> for Item {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Item {
            item_id: row.get("item_id")?,
            name: row.get("name")?,
            file_pass: row.get("file_pass")?,
            listen_count: row.get("listen_count")?,
            datecreate: row.get("datecreate")?,
        })
    }
}


// Item And ShowNote impl
impl QueryableByName<DB> for ItemAndShowNote {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(ItemAndShowNote {
            item_id: row.get("item_id")?,
            name: row.get("name")?,
            file_pass: row.get("file_pass")?,
            listen_count: row.get("listen_count")?,
            datecreate: row.get("datecreate")?,
            note: row.get("note")?,
            note_long: row.get("note_long")?,
        })
    }
}

// ShowNote impl
impl QueryableByName<DB> for ShowNote {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(ShowNote {
            item_id: row.get("item_id")?,
            note: row.get("note")?,
            note_long: row.get("note_long")?,
        })
    }
}


// ##### user get test #####
// curl -XGET http://127.0.0.1:8080/user_get
#[get("user_get")]
async fn user_get_api() -> Result<HttpResponse, Error> {
    // DB Connection!!
    let connection: MysqlConnection = db_connection();
    let users: Vec<Form> = sql_query("SELECT login_id, user_pass FROM users",).load(&connection).unwrap();

    println!("user info: {:?}", users);

    Ok(HttpResponse::Ok().into())
}

// ##### item get test #####
// curl -XGET http://127.0.0.1:8080/item_get
// curl -XGET https://127.0.0.1:8080/item_get
#[get("item_get")]
async fn item_get_api() -> Result<HttpResponse, Error> {
    let connection: MysqlConnection = db_connection();
    let items: Vec<Item> = sql_query("SELECT item_id, name, file_pass, listen_count, datecreate FROM items ORDER BY datecreate DESC",).load(&connection).unwrap();

    println!("{:?}", items);
    Ok(HttpResponse::Ok().into())
}


// ### Item And ShowNote get ###
// curl -XGET http://127.0.0.1:8080/all_get
// curl -XGET https://127.0.0.1:8080/all_get
#[get("all_get")]
async fn test_allget() -> Result<HttpResponse, Error> {
    let connection: MysqlConnection = db_connection();
    let datails: Vec<ItemAndShowNote> = sql_query("SELECT items.item_id, items.name, items.file_pass, items.listen_count, items.datecreate, show_notes.note, show_notes.note_long FROM items LEFT OUTER JOIN show_notes ON items.item_id = show_notes.item_id",).load(&connection).unwrap();

    println!("{:?}", datails);
    Ok(HttpResponse::Ok().into())
}

// get userdata from login_id
fn user_from_id_data_access_select(login_id: &String) -> std::vec::Vec<Form>  {
    // DB connection!!
    let connection: MysqlConnection = db_connection();
    let users: Vec<Form> = sql_query("SELECT login_id, user_pass FROM users WHERE login_id = ?",)
        .bind::<Text, _>(login_id)
        .load(&connection).unwrap();
        
        return users
}

// get All Item Info
fn item_allget_data_access() -> std::vec::Vec<Item> {
    // DB Connection!!
    let connection: MysqlConnection = db_connection();
    let items: Vec<Item> = sql_query("SELECT item_id, name, file_pass, listen_count, datecreate FROM items ORDER BY datecreate DESC",).load(&connection).unwrap();

    return items
}

// item Data Insrt
fn item_data_access_insert(name: &String, file_path: &String, count: i32, date: &String) {
    let connection: MysqlConnection = db_connection();
    let query = sql_query("INSERT INTO items(name, file_pass, listen_count, datecreate)VALUES(?, ?, ?, ?)")
        .bind::<Text, _>(name)
        .bind::<Text, _>(file_path)
        .bind::<Integer, _>(count)
        .bind::<Text, _>(date)
        .execute(&connection);

        println!("{:?}",query);
}



// LoginForm
async fn login_form(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut _ctx = Context::new();

    let view = tmpl
        .render("login.html", &_ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
} 


// cont_login
async fn cont_index(session: Session, tmpl: web::Data::<Tera>,  params: web::Form<Form>) -> Result<HttpResponse, Error> {
    // login_ id からデータを取得
    let users: Vec<Form> = user_from_id_data_access_select(&params.login_id.to_string());

    // query empty
    if users.len() == 0 {      
        return Ok(HttpResponse::Ok().content_type("text/html").body(format!(r#"LoginId Or UserPaas mismatched!! Let's try again</br><a href="/login/login_form">back to login</a>"#)))
    }

    let user = &users[0];
    let user_pass = user.user_pass.to_string();
    let login_data = format!("{}{}", &params.user_pass, &params.login_id);

    // password matching
    let pass_match_flg: bool = password_hash_match(&login_data, &user_pass);

    // UserPaas mismatched!!
    if !pass_match_flg {
        return Ok(HttpResponse::Ok().content_type("text/html").body(format!(r#"LoginId Or UserPaas mismatched!! Let's try again</br><a href="/login/login_form">back to login</a>"#)))
    }

    // ##### Login Ok #####

    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        println!("Session Value: {}", count);
        session.set("counter", count + 1)?;     
    } else {
        session.set("counter", 1)?;
        let sess_value = session.get::<i32>("counter")?;
        println!("First Session Value: {:?}", sess_value);
    }

    // items get
    let items: Vec<Item> = item_allget_data_access();

    // Context
    let mut ctx = Context::new();
    ctx.insert("items", &items);
    

    let view = tmpl
        .render("cont_index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
} 

// Generate to HashPassword API
// example: curl -XPOST -d 'login_id=keisuke&user_pass=ohs80340' http://127.0.0.1:8080/hashpass
#[post("/hashpass")]
async fn password_hashing_api(params: web::Form<Form>) -> Result<HttpResponse, Error> {
    //let hash = bcrypt::hash(format!("{}{}", &params.login_id, &params.user_pass)).unwrap();
    //let hash = pasword_hashing(&params.user_pass.to_string());
    let hash = pasword_hashing(&params.user_pass.to_string(), &params.login_id.to_string());

    Ok(HttpResponse::Ok().body(format!("hash PassWord: {:?}", &hash)))
}

// pasword matching testAPI
// example: curl -XPOST -d 'login_id=keisuke&user_pass=ohs80340' http://127.0.0.1:8080/hash_test
#[post("hash_test")]
async fn pass_mathing(params: web::Form<Form>) -> Result<HttpResponse, Error> {
    let pass_data = format!("{}{}", &params.user_pass, &params.login_id);
    let macth_flg: bool = password_hash_match(&pass_data, &"$2b$10$ujan9bZbu0gqR4u2lS4CZeIoUSe/8mbs2kTlGeip8mGDOBWYCMwFy".to_string());

    Ok(HttpResponse::Ok().body(format!("matching Result: {}", macth_flg)))
}

// Generate to HashPassword Function!!
fn pasword_hashing(password: &String, login_id: &String) -> String {
    bcrypt::hash(format!("{}{}", password, login_id)).unwrap()
}

// matching to password!!
fn password_hash_match(password_input: &String, password_db: &String) -> bool {
    bcrypt::verify(password_input, password_db)
}

// Data Upload and DB Regist
async fn save_file(mut payload: Multipart, tmpl: web::Data::<Tera>) -> Result<HttpResponse, Error> {
   let mut in_name = String::from("");
   let mut in_filepath = String::from("");
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        // split is extention
        let v: Vec<&str> = filename.split('.').collect();
        let name = v[0];
        let extension = v[1];
        in_name = name.to_string();
        // get now time
        let local_datetime = Utc::now().format("%Y%m%d%H%M%S").to_string();
        // filename is fix
        let filename_fix = format!("{}_{}.{}", name, local_datetime, extension);
        // upload path
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename_fix));
        in_filepath = filepath.to_string();
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    // get a now
    let in_date = Utc::now().format("%Y%m%d%H%M%S").to_string();
    // DB Insert
    item_data_access_insert(&in_name, &in_filepath, 0, &in_date);

    // items get
    let items: Vec<Item> = item_allget_data_access();

    let mut ctx = Context::new();
    ctx.insert("items", &items);

    let view = tmpl
        .render("cont_index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

// cont_datails
async fn cont_datails(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut _ctx = Context::new();

    let view = tmpl
        .render("", &_ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))

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


// main Function 
//Server routing
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    async_std::fs::create_dir_all("./tmp").await?;

    let templates = Tera::new("templates/**/*").unwrap();

    // https port
    let ip = "0.0.0.0:443";

    // http port
    //let ip = "0.0.0.0:80";
    
    // https support!!
    // Key load!!
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("pem_key/server-key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("pem_key/server-crt.pem").unwrap();

    println!(" ┌──────────────────────────────────────────────────┐");
    println!(" │                                                  │");
    println!(" │   Serving!                                       │");
    println!(" │                                                  │");
    println!(" │   - On Your Network:  {}               │", ip);
    println!(" │                                                  │");
    println!(" │   Copied local address to clipboard!             │");
    println!(" │                                                  │");
    println!(" └──────────────────────────────────────────────────┘");

    HttpServer::new(move || {
        App::new().data(templates.clone())
        // enable logger - always register actix-web Logger middleware last
        .wrap(middleware::Logger::default())
        // cookie session middleware
        .wrap(CookieSession::signed(&[0; 32]).secure(false))
        .service(
            web::scope("/serach")
                //.route("/serach.html", web::get().to(search)),
        )
        .service(
            web::scope("/regist")
            .route("/save_file", web::post().to(save_file)),
           
        )
        .service(
            web::scope("/login")
            .route("/login_form", web::get().to(login_form))
            .route("/cont_index", web::post().to(cont_index))
            .route("/cont_datails", web::get().to(cont_datails))
        )
        // favicon
        .service(favicon)
        // Generate to HashPassword API
        .service(password_hashing_api)
        .service(pass_mathing)
        .service(user_get_api)
        .service(item_get_api)
        .service(test_allget)
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
                )
        )
              
    })
    // https support!!
    .bind_openssl(ip, builder)
    //.bind(ip)  // http mode
    .expect("Can not bind to port 443")
    .run()
    .await
}