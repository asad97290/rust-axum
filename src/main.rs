use axum::{
    routing::{get, post},
    http::StatusCode,
    Router,
    Json,
};
use serde::{Serialize,Deserialize};

#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
    id: u128
}
#[derive(Deserialize)]
struct UserDto {
    name: String,
    age: u8
}

async fn create_user(
    Json(payload): Json<UserDto>,
) -> Result<Json<User>, StatusCode> { // Changed the return type
    Ok(Json(User {
        name: payload.name,
        age: payload.age,
        id: 12345
    }))
}

async fn get_user() -> Result<Json<User>, StatusCode> { // Changed the return type
    Ok(Json(User {
        name: "Asad".to_string(),
        age: 26,
        id: 12345
    }))
}

#[tokio::main]
async fn main() {
    

    let app = Router::new()
        .route("/user/getUser", get(get_user))
        .route("/user/createUser", post(create_user));

    println!("server running on port 3000");

    let listener_res = tokio::net::TcpListener::bind("0.0.0.0:3000").await;
    let listener = match listener_res {
        Ok(listener)=> listener,
        Err(err)=>{
            println!("{}",err);
            return 
        }
    };
    let server = axum::serve(listener, app).await;
    match server {
        Ok(server)=>server,
        Err(err)=> {
            
            println!("{}",err);
            return
        }
    }
}
