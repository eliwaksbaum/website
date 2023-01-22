use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::Mailbox;
use rocket::form::Form;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(FromForm, Debug)]
pub struct Email<'a>
{
    sender_name: &'a str,
    sender_address: &'a str,
    message: &'a str
}

pub fn contact(form: Form<Email<'_>>) -> Result<()>
{
    let from = (form.sender_name.to_string() + "<eli@waksbaum.com>").parse::<Mailbox>()?;
    let to = "Eli <eli@waksbaum.com>".parse::<Mailbox>()?;

    let email = Message::builder() 
        .from(from) 
        .to(to) 
        .subject("Contact Form from eli.waksbaum.com") 
        .body(form.message.to_string() + "\n\nSent from: " + form.sender_address)?; 

    let password = fs::read_to_string("/home/eli/password.secret")?.replace(char::is_whitespace, "");
    let creds = Credentials::new("eli@waksbaum.com".to_string(), password); 

    // Open a remote connection
    let mailer = SmtpTransport::relay("smtp.migadu.com")? 
        .credentials(creds) 
        .build(); 

    // Send the email 
    mailer.send(&email)?;

    Ok(())
}