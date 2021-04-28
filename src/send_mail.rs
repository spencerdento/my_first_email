use lettre::Transport;


extern crate lettre;
extern crate lettre_email;

pub fn _send_email(to: &str, name: &str) {
    let email = lettre_email::EmailBuilder::new()
      .to((to, name))
      .from(("micromessager@gmail.com", "Test Email"))
      .subject("Sent from my uMessage-r")
      .text("Hi, what're you up to today?")
      .build()
      .unwrap();

    let mut mailer = lettre::SmtpClient::new_simple("smtp.gmail.com")
      .unwrap()
      .credentials(lettre::smtp::authentication::Credentials::new("micromessager@gmail.com".into(), "MicroMessagerT3st".into()))
      .transport();
    
    let result = mailer.send(email.into());

    println!("{:?}", result);
}