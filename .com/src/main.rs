#[macro_use] extern crate rocket;
use rocket::fs::{ FileServer, NamedFile };

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open(".com/404.html").await.ok()
}

#[catch(403)]
async fn permission_denied() -> Option<NamedFile> {
    NamedFile::open(".com/403.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(".com/public/"))
        .register("/", catchers![not_found, permission_denied])
}