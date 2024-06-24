use std::sync::Arc;

use axum::{response::Html, routing::get, Router};
use db::{setup_db, Db};
use surrealdb::opt::auth::Database;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

mod db;


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // ServeDir falls back to serve index.html when requesting a directory
    let db = setup_db().await.unwrap();
    println!("{:?}", db.version());
    let database = Arc::new(Mutex::new(db));

    let state = StateHolder{database};

    let router = Router::new().nest_service("/", ServeDir::new("assets"));
    let router = router.with_state(state);
    
    Ok(router.into())
}

#[derive(Clone)]
struct StateHolder{
    database: Arc<Mutex<db::Db>>,
}

// use async_trait::async_trait;
// use serde::Serialize;
// use shuttle_service::{resource::Type, Error, Factory, IntoResource, ResourceBuilder};

// #[derive(Default, Serialize)]
// pub struct Builder {
//     name: String,
// }

// struct DatabaseOrigin{
//     database: db::Db,
// }

// impl Builder {
//     /// Name to give resource
//     pub fn name(mut self, name: &str) -> Self {
//         self.name = name.to_string();
//         self
//     }
// }

// #[async_trait]
// impl ResourceBuilder for Builder {
//     const TYPE: Type = Type::Custom;
//     type Config = Self;
//     type Output = String;

//     fn config(&self) -> &Self::Config {
//         self
//     }

//     async fn output(self, _factory: &mut dyn Factory) -> Result<Self::Output, Error> {
//         // factory can be used to get resources from Shuttle
//         Ok(self.name)
//     }
// }

// #[async_trait]
// impl IntoResource<DatabaseOrigin> for Db {
//     async fn into_resource(self) -> Result<DatabaseOrigin, Error> {
//         Ok(DatabaseOrigin { database: self })
//     }
// }
