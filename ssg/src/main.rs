use heml::build;
use blog::prepare_previews;

pub mod heml;
pub mod blog;

fn main()
{
    println!("=============");
    println!("Running SSG");
    
    println!("  Building Docs");
    build("docs/", "../.com/");
    println!("  ..Done");

    println!("  Preparing Blog Previews");
    prepare_previews("../.com/public/blog/", "eags/blog-preview.heml", "../.com/assets/blog-previews.toml");
    println!("  ..Done");

    println!("..Done");
    println!("=============");
}