use std::fs;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf, Display};

type EagTable = HashMap<String, Eag>;
#[derive(Deserialize)]
struct Eag
{
    path: String,
    text_params: Option<Vec<String>>,
    file_params: Option<Vec<String>>
}
type EagRealization = HashMap<String, String>;
type EagArgs = HashMap<String, String>;
type FileCache= HashMap<String, String>;

fn generate_eag_realization(eag: &Eag, args: &EagArgs, inside_text: &str, cache: &mut FileCache) -> EagRealization
{
    let mut rlz = EagRealization::new();

    match &eag.text_params
    {
        Some(params) => {
            place_text_params_into_realization(&params, args, &mut rlz)
                .unwrap_or_else(|param| panic!("missing param"));
        }
        None => {}
    }

    match &eag.file_params
    {
        Some(params) => {
            place_file_params_into_realization(&params, args, &mut rlz, cache)
                .unwrap_or_else(|param| panic!("missing param"));
        },
        None => {}
    }

    rlz.insert(String::from("%@%inside%@%"), inside_text.to_string());

    rlz
}

fn place_text_params_into_realization<'a>(params: &'a Vec<String>, args: &EagArgs, rlz: &mut EagRealization) -> Result<(), &'a str>
{
    for p in params
    {
        match args.get(p)
        {
            Some(arg) => { rlz.insert(text_paramify(p), arg.clone()); }
            None => { return Err(p); }
        }
    }
    Ok(())
}

fn text_paramify(p: &str) -> String
{
    String::from("%@%") + p + "%@%"
}

fn place_file_params_into_realization<'a>(params: &'a Vec<String>, args: &EagArgs, rlz: &mut EagRealization, cache: &mut FileCache) -> Result<(), &'a str>
{
    for p in params
    {
        match args.get(p)
        {
            Some(path) => {
                let file_text = cache.entry(path.clone()).or_insert_with(|| read_insert(&path));
                rlz.insert(file_paramify(p), file_text.clone());
            },
            None => { return Err(p); }
        }
    }
    Ok(())
}

fn read_insert(path: &str) -> String
{
    let insert_path = String::from("inserts/") + path;
    return fs::read_to_string(&insert_path).unwrap_or_else(|_| panic!("Could not open {}.", &insert_path));
}

fn file_paramify(p: &str) -> String
{
    String::from("@%@") + p + "@%@"
}

fn replace(page: &str, rlz: &EagRealization) -> String
{
    let mut next = page.to_string();
    for (param, arg) in rlz
    {
        next = next.replace(param, arg);
    }
    next
}