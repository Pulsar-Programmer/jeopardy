use actix_web::{get, HttpResponse, Responder};


#[macro_export]
macro_rules! website {
    ($($i:ident; $e:expr),+) => {
        $(
            pub const $i: &'static str = include_str!(concat!("../src-web/", $e, ".html"));
        )*
    };
}
website!(
    HOMEPAGE; "homepage",
    JOIN; "join",
    PLAY; "play",
    HOST; "host"
);


///This is the `index` endpoint for the browser.
#[get("/")]
pub async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(HOMEPAGE)
}

///This is the `join` endpoint for the browser.
#[get("/join")]
pub async fn join() -> impl Responder{
    HttpResponse::Ok().body(JOIN)
}



///This is the `play` endpoint simply to display the page for the browser.
#[get("/play")]
pub async fn play() -> impl Responder{
    HttpResponse::Ok().body(PLAY)
}

///This is the `host` endpoint simply to display the page for the browser.
#[get("/host")]
pub async fn host() -> impl Responder{
    HttpResponse::Ok().body(HOST)
}


// ws://play to join as a player
// ws://host to join as a host



//make room
//join room

//buzzer system
// #[get()]
// pub async fn start_buzzer() -> impl Responder{

// }