use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn db_connection() -> MysqlConnection {
    let mysql_connection_result: MysqlConnection;
    let database_url = "mysql://ohs80340:ohs80340@127.0.0.1:3306/podcast_db";
    mysql_connection_result = MysqlConnection::establish(&database_url)
        .expect(&format!(" Error connecting to {} ", database_url)); 

        let url_length: usize = database_url.len();
        let mut spece: String = "".to_string();
        let s_count: usize = 66usize - url_length;

        for _n in 1..s_count {
            spece = spece + " ";
        }

    println!(" ┌──────────────────────────────────────────────────────────────────┐");
    println!(" │                                                                  │");
    println!(" │ DB Connectiong OK!                                               │");
    println!(" │                                                                  │");
    println!(" │ - DB conect info:                                                │");
    println!(" | {}{}|", database_url, spece);
    println!(" │                                                                  │");
    println!(" └──────────────────────────────────────────────────────────────────┘");

        mysql_connection_result
}