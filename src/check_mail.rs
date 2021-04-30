use std::{fs, net::TcpStream};
use anyhow::Error;
use imap::Session;
use native_tls::{TlsConnector, TlsStream};

pub fn email_login(domain: &str, username: &str, password: &str) -> anyhow::Result<Session<TlsStream<TcpStream>>> {
    let tls = TlsConnector::builder().build()?;
    //make a new client at the address of the domain and port, double check with domain, and give it a TLS connector
    let client = imap::connect((domain, 993), domain, &tls)?;
    
    //now i start my session
    match client.login(username, password) {
        Ok(x) => Ok(x),
        Err(a) => Err(Error::new(a.0))
    }
}

pub fn get_latest_email(my_session: &mut Session<TlsStream<TcpStream>>) {

    //select my inbox and get the number of messages
    let inbox_len = my_session.select("INBOX").expect("Couldn't find my INBOX").exists;

    let my_fetch = my_session.fetch(inbox_len.to_string(), "RFC822").expect("Couldn't Find Messages");

    for mail in my_fetch.iter() {
        if let Some(body) = mail.body() {
            fs::write("email.txt", body).expect("File Error");
        } else {
            println!("Message was unreadable");
        }
    }
}