use std::env::var;

pub fn get_database_url() -> String {
    var("DATABASE_URL").expect("Expected a DATABASE_URL")
}

pub fn get_queue_url() -> String {
    var("QUEUE_URL").expect("Expected a QUEUE_URL")
}

pub fn get_server_port() -> String {
    var("SERVER_PORT").unwrap_or(String::from("5005"))
}

pub fn get_previews_path() -> String {
    var("PREVIEWS_DIR").unwrap_or("./previews".to_string())
}
