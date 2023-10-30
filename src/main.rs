/**
* This is a simple Actix-Web based MySQL connection template
*/

// -------------- Crates --------------
use actix_web::{get, App, HttpServer, HttpResponse, Responder}; // You may want to use web too, but template doesn't need it
use actix_files::Files;
use sqlx_mysql;
use sqlx::MySqlConnection;
use sqlx_mysql::MySqlRow; 
use sqlx::Connection;
use sqlx::Executor;
use sqlx::Row;

/* ENDPOINTS */

#[get("/basic_data")] // may need to remove the last /
async fn basic_data() -> impl Responder {
    let mut conn: MySqlConnection = 
    MySqlConnection::connect("mysql://root:@localhost/rust_test_data")
    .await
    .unwrap();
    let result: MySqlRow = 
    conn.fetch_one(sqlx::query("select data from basic")).await.unwrap();

    // turn MySQLRow into string
    let data: String = result.get::<String, _>("data");
    
    HttpResponse::Ok().body(data)
} 

/* ENDPOINTS END */

// -------------- Main --------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move|| { 
        App::new()
            .service(basic_data) // Add the endpoint
            // Add index.html file to the root directory of the server
            .service(Files::new("/", "src").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")? 
    .run()
    .await
}