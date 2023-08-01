use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    pub name: String,
    pub redirect: String
}

pub type Router = Vec<Route>;

pub fn get_routes() -> Router {
    let routes_data = fs::read_to_string("routes.json")
        .expect("Please create and configure routes json!");

    serde_json::from_str(&routes_data).expect("Error parsing routes")
}
