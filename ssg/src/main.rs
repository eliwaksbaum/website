use heml::build;

pub mod heml;

fn main()
{
    build("docs/", "../.com/public/");
    println!("Build Succesful.")
}