#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::content::RawHtml;
use rocket::http::Status;
use rocket::serde::Deserialize;
use std::fs;
use std::collections::{HashSet, HashMap};
use std::io::{Error, ErrorKind};
use rand::{seq::SliceRandom, thread_rng};

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
    html: String,
    tags: HashSet<String>
}

#[get("/blog")]
fn blog_home() -> Result<RawHtml<String>, String>
{
    blog(None)
}

#[get("/blog/tag/<tag>")]
fn blog_tag(tag: &str) -> Result<RawHtml<String>, String>
{
    blog(Some(tag))
}

fn blog(tag: Option<&str>) -> Result<RawHtml<String>, String>
{
    fs::read_to_string("assets/blog-home.html").and_then(|mut home_html|{
        fs::read_to_string("assets/blog-previews.toml").and_then(|toml_text| {
            toml::from_str(&toml_text).and_then(|table:PreviewTable|
            {
                home_html = home_html.replace("PREVIEWS_HERE", &get_tagged_previews(&table.previews, tag));
                home_html = home_html.replace("TAGS_HERE", &get_tag_sidebar(table.previews));
                Ok(RawHtml(home_html))
            })
            .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
        })
    })
    .map_err(|e| e.to_string())
}

fn get_tagged_previews(all_previews: &Vec<Preview>, tag: Option<&str>) -> String
{
    all_previews.iter()
        .filter(|p| match tag {
            None => true,
            Some(t) => p.tags.contains(t)
        })
        .fold(String::new(), |a, b| a + &b.html)
}

fn get_tag_sidebar(previews: Vec<Preview>) -> String
{
    let mut counts = HashMap::new();
    let all_tags = previews.into_iter().flat_map(|p| p.tags.into_iter());

    for tag in all_tags
    {
        *counts.entry(tag).or_insert(0) += 1;
    }

    let mut counts = counts.into_iter()
        .collect::<Vec<(String, usize)>>();
    counts.sort_by_key(|(_, count)| usize::max_value() - count);

    counts.into_iter()
        .map(|(tag, count)| format!("<div class=\"tag\"><a class=\"blog\" href=\"/blog/tag/{}\">{} ({})</div>", tag, tag, count))
        .reduce(|a, b| a + &b)
        .unwrap_or_default()
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount("/", routes![shuffle_cards, blog_home, blog_tag])
        .register("/", catchers![not_found, permission_denied])
}