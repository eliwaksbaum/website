#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::content::RawHtml;
use rocket::http::Status;

pub mod blog;
pub mod projects;

#[catch(404)]
async fn not_found() -> Option<NamedFile>
{
    NamedFile::open("assets/404.html").await.ok()
}

#[catch(403)]
async fn permission_denied() -> Option<NamedFile>
{
    NamedFile::open("assets/403.html").await.ok()
}

#[get("/projects")]
fn projects_home() -> Result<RawHtml<String>, Status>
{
    projects::shuffle_cards()
}

#[get("/blog")]
fn blog_home() -> Result<RawHtml<String>, Status>
{
    blog::blog(None)
}

#[get("/blog/tag/<tag>")]
fn blog_tag(tag: &str) -> Result<RawHtml<String>, Status>
{
    blog::blog(Some(tag))
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount("/", routes![projects_home, blog_home, blog_tag])
        .register("/", catchers![not_found, permission_denied])
}