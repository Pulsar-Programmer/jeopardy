use serde::Serialize;
use serde_json::Value;
use surrealdb as s;
use s::Surreal;
use s::opt::auth::{Root, Scope};
use s::engine::local::Mem;
use s::engine::remote::ws::{Client, Ws};

pub type Db = Surreal<s::engine::local::Db>;

pub async fn setup_db() -> s::Result<Db>{
    //Change this into the embedded version when ready for non-data persistence
    let mut db = Surreal::new::<Mem>(()).await?; 

    db.use_ns("jeopardy").use_db("main").await?;
    
    Ok(db)
}


pub async fn query_value(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Vec<s::Result<Vec<Value>>>>{
    query_all(db, surrealql, parameters).await
}


pub async fn query_all<T: std::fmt::Debug + serde::de::DeserializeOwned>(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Vec<s::Result<Vec<T>>>>{
    let mut result = db.query(surrealql).bind(parameters).await?;
    let mut vec: Vec<Result<Vec<T>, _>> = Vec::new();
    for i in 0..result.num_statements(){
        let result: Result<Vec<T>, _> = result.take(i);
        // println!("{result:?}");
        vec.push(result)
    }
    Ok(vec)
}

//make this something used more frequently for querying without a wanted response.
///Querying without a processed response.
pub async fn sole_query(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<s::Response>{
    db.query(surrealql).bind(parameters).await
}

///Only to get the first part of the result of the query.
pub async fn query_once<T: std::fmt::Debug + serde::de::DeserializeOwned>(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Vec<T>>{
    let mut result = db.query(surrealql).bind(parameters).await?;
    let result: Result<Vec<T>, _> = result.take(0);
    result
}

//Stabilize these features ASAP
// Extract looks better for error handling these separately and getting an owned copy hoenstly but ig the latter should be sued sicne it alwaus could be used how this oen is.
pub fn extract_first<T>(mut vec: Vec<T>) -> Option<T>{
    if vec.is_empty(){
        return None;
    }
    Some(vec.remove(0))
}

pub async fn query_once_option<T: std::fmt::Debug + serde::de::DeserializeOwned>(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Option<T>>{
    let mut result = db.query(surrealql).bind(parameters).await?;
    let result: Result<Option<T>, _> = result.take(0);
    result
}