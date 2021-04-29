use lettre::{SmtpTransport, Transport, smtp::response::Response};
use lettre::smtp::error::Error;

extern crate lettre;
extern crate lettre_email;

pub fn _send_email(to: &str, name: &str, domain: &str, username: &str, password: &str) -> Result<Response, Error> {
    let email = lettre_email::EmailBuilder::new()
      .to((to, name))
      .from(("micromessager@gmail.com", "Test Email"))
      .subject("Sent from my uMessage-r")
      .text("Hi, what're you up to today?")
      .build()
      .unwrap();

    let mut mailer = make_smtp_transport(domain, username, password)?;
    
    let result = mailer.send(email.into())?;
    Ok(result)
}

fn make_smtp_transport(domain: &str, username: &str, password: &str) -> Result<SmtpTransport, Error> {
    let mailer = lettre::SmtpClient::new_simple(domain)?;
    let mailer = mailer.credentials(lettre::smtp::authentication::Credentials::new(username.into(), password.into())).transport();

    Ok(mailer)
}