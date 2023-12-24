use actix_web::{post, web};


#[post("/server/fav")]
fn favserver() -> Responder {
    //Todo: favourte a server.
}


fn database() {
    //Todo: Database setup.
}

#[post("/server/remove")]
fn favserver() -> Responder {
    //Todo: Removes a favourte server.
}

#[post("/server/list")]
fn favserver() -> Responder {
    //Todo: List the favoute servers
}


