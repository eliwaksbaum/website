use serde::Serialize;
use std::fs;
use std::iter::once;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Serialize)]
struct Preview
{
    order: usize,
    html: String
}

struct HtmlInfo<'a>
{
    header: &'a str,
    date: NaiveDate,
    tags: &'a str,
    prev_start: usize
}

pub fn prepare_previews(blog_dir: &str, template_path: &str, write_path: &str)
{
    fs::write(write_path, gen_preview_reference(blog_dir, template_path))
        .unwrap_or_else(|_| panic!("Couldn't write to {}", write_path));
}


fn gen_preview_reference(blog_dir: &str, template_path: &str) -> String
{
    let template = fs::read_to_string(template_path).expect("Can't find blog preview template.");
    let blog_dir = PathBuf::from(blog_dir);

    let mut previews = Vec::<(String, NaiveDate)>::new();
    for entry in fs::read_dir(&blog_dir).expect("Can't find blog html dir.")
    {
        let post = entry.expect("Something went wrong in the DirEntry iterator.");
        previews.push(gen_preview_html(&post.path(), &template, &blog_dir));
    }

    previews.sort_by_key(|(_, date)| *date);
    let previews = previews.iter().rev()
        .enumerate()
        .map(|(i, (text, _))| Preview { order: i, html: text.to_string() })
        .collect::<Vec<Preview>>();
    let table = HashMap::from([("previews", previews)]);

    toml::to_string(&table).expect("Couldn't serialize the Previews Vector.")
}

fn gen_preview_html(path: &Path, template: &str, dir: &Path) -> (String, NaiveDate)
{
    let html = fs::read_to_string(path).unwrap();
    let info = get_info(&html)
        .unwrap_or_else(|search| panic!("Reconaissance failed while looking for \"{}\" in \"{:?}\".", search, path));

    
    let link = path.strip_prefix(dir).expect("The blog dir prefix won't strip from a blog file.").as_os_str();
    //let prev = &html[info.header_close+1..info.header_close+100];
    let prev = &html[info.prev_start..]
        .split_ascii_whitespace()
        .take(50)
        .chain(once("..."))
        .fold(String::new(), |a ,b| a + b + " ");

    let rlz = HashMap::from([
        ("{{header}}", info.header),
        ("{{link}}", link.to_str().expect("There's a blog whose link isn't UTF-8.")),
        ("{{preview}}", prev),
        ("{{tags}}", info.tags)
    ]);

    (insert(template, &rlz), info.date)
}

fn get_info<'a>(html: &'a str) -> Result<HtmlInfo<'a>, &str>
{
    let h1_open = match html.find("<h1>") {
        Some(i) => i,
        None => { return Err("The <h1> opening tag"); }
    };

    let date_open_open = match html.find("<div class=\"date\">") {
        Some(i) => i,
        None => { return Err("The date <div class=\"date\"> opening tag"); }
    };
    let date_close_open = match (&html[date_open_open..]).find("</div>") {
        Some(i) => date_open_open + i,
        None => { return Err("The date </div> closing tag") }
    };

    let tags_open = match html.find("<div class=\"tags\">") {
        Some(i) => i,
        None => { return Err("The tags <div class=\"tags\"> opening tag"); }
    };
    let tags_close = match (&html[tags_open..]).find("</div>") {
        Some(i) => tags_open + i + 6,
        None => { return Err("The tags </div> closing tag") }
    };

    let a_header = &html[h1_open..=(date_close_open + 6)];
    let a_date = NaiveDate::parse_from_str(&html[(date_open_open + 18)..date_close_open], "%m/%d/%Y").unwrap();
    let a_tags = &html[tags_open..=tags_close];

    Ok(HtmlInfo { header: a_header, date: a_date, tags: a_tags, prev_start: date_close_open + 7 })
}

fn insert(template: &str, rlz: &HashMap<&str, &str>) -> String
{
    let mut prev = template.to_string();
    for (param, arg) in rlz
    {
        prev = prev.replace(param, arg);
    }
    prev
}