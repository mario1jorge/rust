use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Item {
    id: String,
    name: String,
    description: String,
}

type Items = Mutex<HashMap<String, Item>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let items = web::Data::new(Items::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(items.clone())
            .route("/items", web::get().to(get_items))
            .route("/items/{id}", web::get().to(get_item))
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_items(items: web::Data<Items>) -> impl Responder {
    let items = items.lock().unwrap();
    let items: Vec<&Item> = items.values().collect();
    HttpResponse::Ok().json(items)
}

async fn get_item(items: web::Data<Items>, id: web::Path<String>) -> impl Responder {
    let items = items.lock().unwrap();
    if let Some(item) = items.get(&id.into_inner()) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn create_item(items: web::Data<Items>, new_item: web::Json<Item>) -> impl Responder {
    let mut items = items.lock().unwrap();
    if items.contains_key(&new_item.id) {
        HttpResponse::BadRequest().body("Item already exists")
    } else {
        items.insert(new_item.id.clone(), new_item.into_inner());
        HttpResponse::Created().finish()
    }
}

async fn update_item(
    items: web::Data<Items>,
    id: web::Path<String>,
    updated_item: web::Json<Item>,
) -> impl Responder {
    let mut items = items.lock().unwrap();
    if items.contains_key(&id.into_inner()) {
        items.insert(updated_item.id.clone(), updated_item.into_inner());
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_item(items: web::Data<Items>, id: web::Path<String>) -> impl Responder {
    let mut items = items.lock().unwrap();
    if items.remove(&id.into_inner()).is_some() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
