use actix_multipart::Multipart;
//use actix_web::{ HttpRequest, get, post, guard, Result, error, middleware, web, App, Error, HttpResponse, HttpServer, Responder };
use actix_web::{ get, post, guard, Result, error, middleware, web, App, Error, HttpResponse, HttpRequest, HttpServer};
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
use std::path::PathBuf;

use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use tera::{ Tera, Context };

extern crate tera;

//use once_cell::sync::Lazy;
//use once_cell::sync::OnceCell;

mod mysql_connection;
//mod errors;

type DB = diesel::mysql::Mysql;


// macro

// Rounding
macro_rules! _round {
    ($x:expr, $scale:expr) => (($x * $scale).round() / $scale)
}
// Round up
macro_rules! ceil {
    ($x:expr, $scale:expr) => (($x * $scale).ceil() / $scale)
}
// Truncate
macro_rules! _floor {
    ($x:expr, $scale:expr) => (($x * $scale).floor() / $scale)
}

// macro


// items Data
#[derive(Deserialize, Serialize, Debug,  Clone)]
pub struct Item {
    item_id: i32,
    name: Option<String>,
    file_pass: Option<String>,
    listen_count: Option<i32>,
    datecreate: Option<String>,
}

// Item And ShowNote
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemAndShowNote {
    item_id: i32,
    name: String,
    file_pass: Option<String>,
    listen_count: Option<i32>,
    datecreate: Option<String>,
    note: Option<String>,
    note_long: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemAndShowNoteForUpdateForm {
    item_id: i32,
    name: String,
    listen_count: i32,
    note: String,
    note_long: String,
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

// item count
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemCount {
    item_count: i64,
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

// item count impl
impl QueryableByName<DB> for ItemCount {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(ItemCount {
            item_count: row.get("item_count")?,
        })
    }
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


// get Limit 30 Item Info
fn item_and_shownote_limitget_data_access(page_number: i32) -> std::vec::Vec<ItemAndShowNote> {

    let mut _st_get_number: i32 = 0i32;

    if page_number == 1 {
        _st_get_number = 0i32;
    } else {
        _st_get_number = (page_number - 1i32) * 30i32;
    }

    // DB Connection!!
    let connection: MysqlConnection = db_connection();
    let items: Vec<ItemAndShowNote> = sql_query("SELECT
	                                                items.item_id AS item_id,
	                                                items.name AS name,
	                                                items.file_pass AS file_pass,
	                                                items.listen_count AS listen_count,
	                                                items.datecreate AS datecreate,
	                                                show_notes.note AS note,
	                                                show_notes.note_long AS note_long
                                                FROM
	                                                items
                                                LEFT OUTER JOIN show_notes ON
	                                                items.item_id = show_notes.item_id 
	                                            LIMIT ?, 30",)
                                    .bind::<Integer,_>(_st_get_number)
                                    .load(&connection)
                                    .unwrap();

    return items;
}

// get item count
fn item_all_count() -> i64 {
    // DB Conection!!
    let connection: MysqlConnection = db_connection();
    let item_count_v: Vec<ItemCount> = sql_query("SELECT 
	                                                COUNT(items.item_id) AS item_count
                                                FROM
	                                                items",)
    .load(&connection).unwrap();

    // get in Vec
    let item_count_s = &item_count_v[0];
    // get in Struct
    let item_count: i64 = item_count_s.item_count;

    return item_count   
}


// get ItemAndShowNote By item_id
fn item_get_by_id(item_id: &i32) -> std::vec::Vec<ItemAndShowNote> {
    // DB Connection!!
    let connection: MysqlConnection = db_connection();
    let items: Vec<ItemAndShowNote> = sql_query("SELECT
	                                                items.item_id AS item_id,
	                                                items.name AS name,
	                                                items.file_pass AS file_pass,
	                                                items.listen_count AS listen_count,
	                                                items.datecreate AS datecreate,
	                                                show_notes.note AS note,
	                                                show_notes.note_long AS note_long
                                                FROM
	                                                items
                                                LEFT OUTER JOIN show_notes ON
	                                                items.item_id = show_notes.item_id 
	                                                WHERE items.item_id = ?",)
    .bind::<Integer, _>(item_id)
    .load(&connection).unwrap();

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


fn show_notes_data_access_insert(item_id: &i32, note: &String, note_long: &String) {
    // DB Connection!!
    let connection: MysqlConnection = db_connection();
    let query = sql_query("INSERT INTO
	                            show_notes(
                                item_id,
	                            note,
	                            note_long
                           )VALUES(
                                 ?,
                                 ?,
                                 ?)")
    .bind::<Integer, _>(item_id)
    .bind::<Text, _>(note)
    .bind::<Text, _>(note_long)
    .execute(&connection);
                         
    println!("{:?}",query);

}

// items UPDATE!!
fn items_data_access_update_by_id(item_id: &i32, name: &String, public_flg: &i32) {
    // DB connection!!
    let connection: MysqlConnection = db_connection();
    let query = sql_query("UPDATE 
	                            items 
                           SET
	                            items.name = ?,
                                items.listen_count = ?
                           WHERE 
	                            item_id = ?")
    .bind::<Text, _>(name)
    .bind::<Integer, _>(public_flg)
    .bind::<Integer, _>(item_id)
    .execute(&connection);

    println!("{:?}", query);
}

// items Delete!!
fn _items_data_accese_delete_by_id(item_id: &i32) {

    // DB connection!!
    let connection: MysqlConnection = db_connection();
    let query = sql_query("DELETE
                           FROM
                                items
                           WHERE
                                item_id = ?")
    .bind::<Integer, _>(item_id)
    .execute(&connection);

    println!("{:?}", query);
}

// show_notes Delete!!
fn show_notes_data_access_delete_by_id(item_id: &i32) {
    // DB connection!!
    let connection: MysqlConnection = db_connection();
    let query = sql_query("DELETE 
                           FROM 
                                show_notes
                           WHERE
                                item_id = ?")
    .bind::<Integer, _>(item_id)
    .execute(&connection);
    
    println!("{:?}", query);
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

    // items ALL get not use
    //let items: Vec<Item> = item_allget_data_access();

    let item_count_i64: i64 = item_all_count();
    let item_count_f64 = item_count_i64 as f64;
    let mut _page_count: f64 = 0.0;

    if item_count_i64 <= 1 {
        _page_count = 1f64;
    } else {
        _page_count = item_count_f64 / 30f64;
    }

    let page_count_roundup = ceil!(_page_count, 1f64);
    let page_count_roundup_i64: i64 = page_count_roundup as i64;

    // items Limit get
    let items: Vec<ItemAndShowNote> = item_and_shownote_limitget_data_access(1);

    let mut page_nation: Vec<i64> = vec![];
    
    for n in 0..page_count_roundup_i64 {
        page_nation.push(n + 1);
    }
    
    // Context
    let mut ctx = Context::new();
    ctx.insert("items", &items);
    ctx.insert("page_count", &page_count_roundup);
    // login first page
    ctx.insert("current_page", &1);
    ctx.insert("page_nation", &page_nation);
    
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

// Data Upload and Psth DB Regist
async fn save_file(mut payload: Multipart, tmpl: web::Data::<Tera>) -> Result<HttpResponse, Error> {
   let mut in_name = String::from("");
   let mut in_filepath_db = String::from("");
   let mut db_flg = true;
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        if "" != filename {
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
            // DB Regist FilePath
            let filepath_db = format!("/tmp/{}", sanitize_filename::sanitize(&filename_fix));
            in_filepath_db =  filepath_db.to_string();

            let mut f = async_std::fs::File::create(filepath).await?;

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).await?;
            }
        } else {
            db_flg = false;
        }

    }

    if true == db_flg {
        // get a now
        let in_date = Utc::now().format("%Y/%m/%d").to_string();
        // DB Insert
        item_data_access_insert(&in_name, &in_filepath_db, 0, &in_date);
    }

    // items get
    //let items: Vec<Item> = item_allget_data_access();

    let item_count_i64: i64 = item_all_count();
    let item_count_f64 = item_count_i64 as f64;
    let mut _page_count: f64 = 0.0;

    if item_count_i64 <= 1 {
        _page_count = 1f64;
    } else {
        _page_count = item_count_f64 / 30f64;
    }

    let page_count_roundup = ceil!(_page_count, 1f64);
    let page_count_roundup_i64: i64 = page_count_roundup as i64;

    // items Limit get
    let items: Vec<ItemAndShowNote> = item_and_shownote_limitget_data_access(1);

    let mut page_nation: Vec<i64> = vec![];
    
    for n in 0..page_count_roundup_i64 {
        page_nation.push(n + 1);
    }

    let mut ctx = Context::new();
    ctx.insert("items", &items);
    ctx.insert("page_count", &page_count_roundup);
    // login first page
    ctx.insert("current_page", &1);
    ctx.insert("page_nation", &page_nation);

    let view = tmpl
        .render("cont_index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

// cont_datails show
async fn cont_datails(tmpl: web::Data<Tera>, query: web::Query<Item>) -> Result<HttpResponse, Error> {
    
    println!("item_id is {}", query.item_id);

    // Item Info Get
    let item_and_shownote: Vec<ItemAndShowNote> = item_get_by_id(&query.item_id);

    // Context
    let mut ctx = Context::new();
    ctx.insert("item_and_shownote", &item_and_shownote);

    println!("item info {:?}", item_and_shownote);

    // Render Template
    let view = tmpl
        .render("cont_datails.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
} 

// index_dtails show
#[get("/index_datails")]
async fn index_datails(tmpl: web::Data<Tera>, query: web::Query<Item>) -> Result<HttpResponse, Error> {
    
    println!("item_id is {}", query.item_id);

    // Item Info Get
    let item_and_shownote: Vec<ItemAndShowNote> = item_get_by_id(&query.item_id);

    // Context
    let mut ctx = Context::new();
    ctx.insert("item_and_shownote", &item_and_shownote);

    println!("item info {:?}", item_and_shownote);

    // Render Template
    let view = tmpl
        .render("index_datails.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
} 

// items and shownote Update!!
async fn cont_datails_update(tmpl: web::Data<Tera>, params: web::Form<ItemAndShowNoteForUpdateForm>) -> Result<HttpResponse, Error> {

    // items name Upload!!
    items_data_access_update_by_id(&params.item_id, &params.name, &params.listen_count);

    // shownote DeleteInsert!!
    // shownote Delete
    show_notes_data_access_delete_by_id(&params.item_id);
    // shownote Insert
    show_notes_data_access_insert(&params.item_id, &params.note, &params.note_long);
   
    // reload
    let item_and_shownote: Vec<ItemAndShowNote> = item_get_by_id(&params.item_id);
    println!("RELOAD RESULT!! {:?}", &item_and_shownote);

    let mut ctx = Context::new();
    ctx.insert("item_and_shownote", &item_and_shownote);
    
    let view = tmpl
        .render("cont_datails.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))   
}

// index.html
#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let item_count_i64: i64 = item_all_count();
    let item_count_f64 = item_count_i64 as f64;
    let mut _page_count: f64 = 0.0;

    if item_count_i64 <= 1 {
        _page_count = 1f64;
    } else {
        _page_count = item_count_f64 / 30f64;
    }

    let page_count_roundup = ceil!(_page_count, 1f64);
    let page_count_roundup_i64: i64 = page_count_roundup as i64;

    // items Limit get
    let items: Vec<ItemAndShowNote> = item_and_shownote_limitget_data_access(1);

    let mut page_nation: Vec<i64> = vec![];
    
    for n in 0..page_count_roundup_i64 {
        page_nation.push(n + 1);
    }

    let mut ctx = Context::new();
    ctx.insert("items", &items);
    ctx.insert("page_count", &page_count_roundup);
    // login first page
    ctx.insert("current_page", &1);
    ctx.insert("page_nation", &page_nation);
    
    
    let view = tmpl
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))   
}

// Audio File Get Api
async fn audio_get(req: HttpRequest) -> Result<fs::NamedFile> {

    let mut path_get = PathBuf::new();
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();

    // Create File Path
    path_get.push("./tmp/");
    path_get.push(path);
  
    Ok(fs::NamedFile::open(path_get)?)
}


// items Limit Get for cont_index
async fn item_get_by_limit_for_cont(req: HttpRequest, tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
       
    let mut query_page_num = String::new();
    let query_str = req.query_string();
    // split for query
    let split_str: Vec<&str> = query_str.split("=").collect();
    // query two over
    if split_str.len() >= 2 {
        query_page_num = split_str[1].to_string();
    } else {
        // noget ?
    }
    let query_page: i32 = query_page_num.parse().unwrap();

    let item_count_i64: i64 = item_all_count();
    let item_count_f64 = item_count_i64 as f64;
    let mut _page_count: f64 = 0.0;

    if item_count_i64 <= 1 {
        _page_count = 1f64;
    } else {
        _page_count = item_count_f64 / 30f64;
    }

    let page_count_roundup = ceil!(_page_count, 1f64);
    let page_count_roundup_i64: i64 = page_count_roundup as i64;

    // items Limit get
    let items: Vec<ItemAndShowNote> = item_and_shownote_limitget_data_access(query_page);

    let mut page_nation: Vec<i64> = vec![];
    
    for n in 0..page_count_roundup_i64 {
        page_nation.push(n + 1);
    }

   let mut ctx = Context::new();
   ctx.insert("items", &items);
    ctx.insert("page_count", &page_count_roundup);
    // login first page
    ctx.insert("current_page", &query_page);
    ctx.insert("page_nation", &page_nation);

   let view = tmpl
        .render("cont_index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

// items Limit Get for index
async fn item_get_by_limit_for_user(req: HttpRequest, tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
       
    let mut _query_page_num = String::new();
    let query_str = req.query_string();
    // split for query
    let split_str: Vec<&str> = query_str.split("=").collect();

    // query two over
    if split_str.len() >= 2 {
        if split_str[1] != "" {
            _query_page_num = split_str[1].to_string();
        } else {
            return Ok(HttpResponse::NotFound().finish())
        }
    } else {
        // noget ?
        return Ok(HttpResponse::NotFound().finish())
    }
    let query_page: i32 = _query_page_num.parse().unwrap();

    let item_count_i64: i64 = item_all_count();
    let item_count_f64 = item_count_i64 as f64;
    let mut _page_count: f64 = 0.0;

    if item_count_i64 <= 1 {
        _page_count = 1f64;
    } else {
        _page_count = item_count_f64 / 30f64;
    }

    let page_count_roundup = ceil!(_page_count, 1f64);
    let page_count_roundup_i64: i64 = page_count_roundup as i64;

    // items Limit get
    let items: Vec<ItemAndShowNote> = item_and_shownote_limitget_data_access(query_page);

    let mut page_nation: Vec<i64> = vec![];
    
    for n in 0..page_count_roundup_i64 {
        page_nation.push(n + 1);
    }

   let mut ctx = Context::new();
   ctx.insert("items", &items);
    ctx.insert("page_count", &page_count_roundup);
    // login first page
    ctx.insert("current_page", &query_page);
    ctx.insert("page_nation", &page_nation);

   let view = tmpl
        .render("index.html", &ctx)
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
            web::scope("/tmp")
                .route("/{filename:.*}", web::get().to(audio_get)),
        )
        .service(
            web::scope("/regist")
            .route("/save_file", web::post().to(save_file))
            .route("/items_update", web::post().to(cont_datails_update)),
           
        )
        .service(
            web::scope("/login")
            .route("/login_form", web::get().to(login_form))
            .route("/cont_index", web::post().to(cont_index))
            .route("/cont_datails", web::get().to(cont_datails))
            .route("/page", web::get().to(item_get_by_limit_for_cont))
            .route("/page_user", web::get().to(item_get_by_limit_for_user)),
        )
        // favicon
        .service(favicon)
        // index.html
        .service(index)
        .service(index_datails)
        //.service(item_get_by_limit_for_back)
        // Generate to HashPassword API
        .service(password_hashing_api)
        .service(pass_mathing)
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