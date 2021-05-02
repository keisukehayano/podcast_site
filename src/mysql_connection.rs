use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
//use dotenv::dotenv;




pub fn db_connection() -> MysqlConnection {
    let database_url = "mysql://ohs80340:ohs80340@127.0.0.1:3306/podcast_db";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))    
}