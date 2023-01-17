#[macro_use] extern crate rocket;
use rocket::fs::{ FileServer, NamedFile };
use rocket::response::content::RawHtml;
use std::fs;
use std::io::{ Error, ErrorKind };
use rand::{ seq::SliceRandom, thread_rng };

#[catch(404)]
async fn not_found() -> Option<NamedFile>
{
    NamedFile::open("404.html").await.ok()
}

#[catch(403)]
async fn permission_denied() -> Option<NamedFile>
{
    NamedFile::open("403.html").await.ok()
}

#[get("/projects")]
fn shuffle_cards() -> Result<RawHtml<String>, Error>
    {
    fs::read_to_string("public/projects/index.html").and_then(|mut html|
    {
            let count = html.matches("order:0").count();
    
            for n in get_order(count)
            {
                html = html.replacen("order:0", &(String::from("order:") + &n.to_string()), 1);
            }
        Ok(RawHtml(html))
    })
}

fn get_order(n: usize) -> Vec<usize>
{
    let mut ordered: Vec<usize> = (0..=n).collect();
    ordered.shuffle(&mut thread_rng());
    ordered
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount("/", routes![shuffle_cards])
        .register("/", catchers![not_found, permission_denied])
}