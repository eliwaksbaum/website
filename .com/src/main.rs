#[macro_use] extern crate rocket;
use rocket::fs::{ FileServer, NamedFile };
use rocket::response::content::RawHtml;
use rocket::http::Status;
use rocket::serde::Deserialize;
use std::fs;
use std::io::{ Error, ErrorKind };
use rand::{ seq::SliceRandom, thread_rng };

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
fn shuffle_cards() -> Result<RawHtml<String>, Status>
{
    fs::read_to_string("assets/projects-home.html").and_then(|mut html|
    {
        let count = html.matches("order:0").count();
    
        for n in get_order(count)
        {
            html = html.replacen("order:0", &(String::from("order:") + &n.to_string()), 1);
        }
        Ok(RawHtml(html))
    })
    .map_err(|_| Status::NotFound)
}

fn get_order(n: usize) -> Vec<usize>
{
    let mut ordered: Vec<usize> = (0..=n).collect();
    ordered.shuffle(&mut thread_rng());
    ordered
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PreviewTable
{
    previews: Vec<Preview>
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Preview
{
    order: usize,
    html: String
}

#[get("/blog")]
fn blog_home() -> Result<RawHtml<String>, Status>
{
    fs::read_to_string("assets/blog-home.html").and_then(|mut home_html|{
        fs::read_to_string("assets/blog-previews.toml").and_then(|toml_text| {
            toml::from_str(&toml_text).and_then(|table:PreviewTable|
            {
                let mut to_insert = String::new();
                for post in table.previews
                {
                    to_insert += &post.html;
                }

                home_html = home_html.replace("PREVIEWS_HERE", &to_insert);
                Ok(RawHtml(home_html))
            })
            .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
        })
    })
    .map_err(|_| Status::NotFound)
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount("/", routes![shuffle_cards, blog_home])
        .register("/", catchers![not_found, permission_denied])
}