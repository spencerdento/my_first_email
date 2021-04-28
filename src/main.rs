mod send_mail;
mod check_mail;

fn main() {
    let mut my_session = check_mail::email_login("imap.gmail.com", "micromessager@gmail.com", "MicroMessagerT3st");
    check_mail::get_latest_email(&mut my_session);

    my_session.logout().unwrap();
    // send_mail::_send_email("micromessager@gmail.com", "Luke Kaufman");
}