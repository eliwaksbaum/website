use rocket::response::content::RawHtml;
use rocket::http::Status;
use std::fs;
use rand::{seq::SliceRandom, thread_rng};

pub fn shuffle_cards() -> Result<RawHtml<String>, Status>
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