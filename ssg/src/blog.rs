use std::fs;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::iter::once;
use chrono::NaiveDate;
use serde::Serialize;
use crate::heml;
use heml::{Eag, EagCall, ListParam, EagArg::{Text, List}};

#[derive(Serialize)]
struct Preview
{
    order: usize,
    html: String,
    tags: Vec<String>
}

struct PreviewPredecessor
{
    date: NaiveDate,
    html: String,
    tags: Vec<String>
}

pub fn prepare_previews(blog_dir: &str, template_path: &str, write_path: &str)
{
    fs::write(write_path, gen_previews_toml(blog_dir, template_path))
        .unwrap_or_else(|_| panic!("Couldn't write to {}", write_path));
}

fn gen_previews_toml(blog_dir: &str, template_path: &str) -> String
{
    let template = fs::read_to_string(template_path).expect("Couldn't read the blog-preview template.");
    let mut preds = gen_preview_preds(&PathBuf::from(blog_dir), &template);
    
    preds.sort_by_key(|p| p.date);
    let previews = preds.into_iter().rev()
        .enumerate()
        .map(|(i, p)| Preview { order: i, html: p.html, tags: p.tags })
        .collect::<Vec<Preview>>();
    
    toml::to_string(&HashMap::from([("previews", previews)])).expect("Couldn't serialize the Previews vector.")
}

fn gen_preview_preds(blog_dir: &Path, template: &str) -> Vec<PreviewPredecessor>
{
    let tags_param = ListParam {
        name: String::from("tags"),
        wrapper: String::from("<a class=\"blog\" href=\"/blog/tag/{{tags}}\">{{tags}}</a>"),
        join: String::from(", ")
    };

    let preview_eag = Eag {
        text_params: Some(vec![String::from("title"), String::from("date")]),
        file_params: None,
        list_params: Some(vec![tags_param]),
        path: String::new()
    };

    fs::read_dir(&blog_dir).expect("Couldn't find blog heml dir.").map(|entry|
    {
        let post = entry.expect("Something went wrong in the DirEntry iterator.");
        let (a_html, call) = get_data(&post.path(), &preview_eag, template);
        
        let a_date = match &call["date"] {
            Text(d) => NaiveDate::parse_from_str(d, "%m/%d/%Y").unwrap(),
            _ => panic!("Couldn't get the post's date from the data in {:?}.", &post.path())
        };
        let a_tags = match &call["tags"] {
            List(l) => l.clone(),
            _ => panic!("Couldn't get the post's tags from the data in {:?}.", &post.path())
        };

        PreviewPredecessor { date: a_date, html: a_html, tags: a_tags }
    })
    .collect::<Vec<PreviewPredecessor>>()
}

fn get_data(post_path: &Path, eag: &Eag, template: &str) -> (String, EagCall)
{
    let heml = fs::read_to_string(post_path)
        .unwrap_or_else(|_| panic!("Could not open {:?}.", post_path));        

    let analysis = heml::parse(&heml)
        .unwrap_or_else(|e| panic!("{}", e));
    
    let mut rlz = heml::generate_eag_realization(&eag, &analysis.eag_call, &carve(&analysis.inside_text), &mut HashMap::new())
        .unwrap_or_else(|param| panic!("Couldn't generate a realization of <<{}>>. Required argument {} is missing or misformatted.", analysis.eag_name, param));
    
    rlz.insert(String::from("{{link}}"), format!("/blog/{}.html", post_path.file_stem().unwrap().to_str().unwrap()));

    (heml::replace(template, &rlz), analysis.eag_call)
}

fn carve(inside: &str) -> String
{
    inside.split_ascii_whitespace()
        .take(50)
        .chain(once("..."))
        .fold(String::new(), |a ,b| a + b + " ")
}