use std::{collections::HashMap, ops::DerefMut, sync::Arc};

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, patch, post}, Json, Router
    
};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Clone, Serialize)]
pub struct Product {
    id:Uuid,
    name: String,
    price: String,
    created_at: String,
    updated_at: String,
}


#[derive(Serialize, Deserialize)]
pub struct NewProduct {
    id: Option<Uuid>,
    name: String,
    price: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}

pub struct ProductListItem {
    id: Uuid,
    product: Product
}

type AppState = Arc<Mutex<HashMap<Uuid, Product>>>;


async fn get_products(State(products):State<AppState>) -> impl IntoResponse {
    let my_products =  products.lock().await;
    match my_products.into() {
        Some(product) => Ok(Json(product.to_owned())),
        None => Err(StatusCode::NOT_FOUND),
    }

}

async fn get_products_by_id(State(products):State<AppState>, Path(id):Path<Uuid>) -> impl IntoResponse{
    let my_products =  products.lock().await;
    match my_products.get(&id) {
        Some(product) => Ok(Json(product.to_owned())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_product(State(products):State<AppState>, Json(new_product): Json<NewProduct>) -> impl IntoResponse {
    let id = Uuid::now_v7();

    let product_to_save = Product {
        id : id,
        name: new_product.name,
        price: new_product.price,
        created_at: Local::now().to_rfc3339(),
        updated_at: Local::now().to_rfc3339(),
    };
    println!("{}", product_to_save.name);
    println!("{}", product_to_save.id);
    products.lock().await.insert(id, product_to_save.clone());
    

    (StatusCode::OK, Json(product_to_save.clone()));
}


async fn update_product_data(mut product:Product, new_data_product:NewProduct){
    product.name = new_data_product.name;
    product.price = new_data_product.price;
    product.updated_at = Local::now().to_rfc3339();
    (StatusCode::OK, Json(product.clone()));
}


async fn update_product(
    State(products):State<AppState>, 
    Path(id):Path<Uuid>, 
    Json(new_product):Json<NewProduct>
) -> impl IntoResponse {

    let my_products =  products.lock().await;
    
    let _ = match my_products.get(&id) {
        Some(product) => Ok(update_product_data(product.clone(), new_product)),
        None => Err(StatusCode::NOT_FOUND)
    };
}


async fn delete_product(State(products):State<AppState>, Path(id):Path<Uuid>) -> impl IntoResponse {
    let mut my_products =  products.lock().await;
    let _ = match my_products.get(&id) {
        Some(product) => Ok(my_products.remove(&id)),
        None => Err(StatusCode::NOT_FOUND)
    };
    (StatusCode::ACCEPTED, format!("Delete Product"))
}

#[tokio::main] //macro tokio
async fn main() {

    let mut products: HashMap<Uuid, Product> = HashMap::new();

    let product = Product {
        id: Uuid::now_v7(),
        name: "Iphone v12".to_string(),
        price: 12.000.to_string(),
        created_at: Local::now().to_rfc3339(),
        updated_at: Local::now().to_rfc3339()
    };

    println!("{}", product.id);
    products.insert(product.id, product);


    let app_state = Arc::new(Mutex::new(products));

    // build our application with a single route
    let app = Router::new()
                        .route("/products", get(get_products))
                        .route("/products/:id", get(get_products_by_id))
                        .route("/products", post(create_product))
                        .route("/products/:id", patch(update_product))
                        .route("/products/:id", delete(delete_product))
                        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


