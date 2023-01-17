use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Serialize)]
struct Preview
{
    order: usize,
    html: String
}

struct HtmlInfo
{
    header_open: usize,
    header_close: usize,
    date: NaiveDate
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

    let header = &html[info.header_open..=info.header_close];
    let prev = &html[info.header_close+1..info.header_close+100];
    let link = path.strip_prefix(dir).expect("The blog dir prefix won't strip from a blog file.").as_os_str();

    let rlz = HashMap::from([
        ("%@%header%@%", header),
        ("%@%link%@%", link.to_str().expect("There's a blog whose link isn't UTF-8.")),
        ("%@%preview%@%", prev),
    ]);

    (insert(template, &rlz), info.date)
}

fn get_info(html: &str) -> Result<HtmlInfo, &str>
{
    let h1_open_open = match html.find("<h1>") {
        Some(i) => i,
        None => { return Err("<h1>"); }
    };
    let date_open_open = match html.find("<div class=\"date\">") {
        Some(i) => i,
        None => { return Err("<div class=\"date\">"); }
    };
    let date_close_open = match (&html[date_open_open..]).find("</div>") {
        Some(i) => date_open_open + i,
        None => { return Err("</div>") }
    };

    let a_date = NaiveDate::parse_from_str(&html[(date_open_open + 18)..date_close_open], "%m/%d/%Y").unwrap();

    Ok(HtmlInfo { header_open: h1_open_open, header_close: date_close_open + 6, date: a_date })
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