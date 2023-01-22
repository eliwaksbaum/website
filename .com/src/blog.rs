use rocket::serde::Deserialize;
use rocket::response::content::RawHtml;
use rocket::http::Status;
use std::fs;
use std::collections::{HashSet, HashMap};

type ChainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

pub fn blog(tag: Option<&str>) -> Result<RawHtml<String>, Status>
{
    generate(tag).map_err(|_| Status::NotFound)
}

fn generate(tag: Option<&str>) -> ChainResult<RawHtml<String>>
{
    let mut home_html = fs::read_to_string("assets/blog-home.html")?;
    let toml_text = fs::read_to_string("assets/blog-previews.toml")?;
    let table: PreviewTable = toml::from_str(&toml_text)?;

    home_html = home_html.replace("PREVIEWS_HERE", &get_tagged_previews(&table.previews, tag));
    home_html = home_html.replace("TAGS_HERE", &get_tag_sidebar(table.previews));
    Ok(RawHtml(home_html))
}

fn get_tagged_previews(all_previews: &Vec<Preview>, tag: Option<&str>) -> String
{
    all_previews.iter()
        .filter(|p| match tag {
            None => true,
            Some(t) => p.tags.iter().map(|x| normalize(x)).find(|x| x.eq(t)).is_some()
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
        .map(|(tag, count)| (normalize(&tag), tag, count))
        .map(|(tag, pretty_tag, count)| format!("<div class=\"tag\"><a class=\"blog\" href=\"/blog/tag/{}\">{} ({})</div>", tag, pretty_tag, count))
        .reduce(|a, b| a + &b)
        .unwrap_or_default()
}

pub fn normalize(tag: &str) -> String
{
    tag.to_lowercase().replace(" ", "_")
}