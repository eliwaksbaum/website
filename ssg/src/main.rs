use heml::build;

pub mod heml;

fn main()
{
    build("docs/", "../.com/");
    println!("Build Succesful.")
}