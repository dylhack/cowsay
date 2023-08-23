use std::env::var;

pub fn get_database_url() -> String {
    var("DATABASE_URL").expect("Expected a DATABASE_URL")
}

pub fn get_server_port() -> String {
    var("SERVER_PORT").unwrap_or(String::from("5005"))
}
