use crate::*;

pub async fn login(session: &mut Session, email_or_tel: &str, password: &str) -> Result<()> {
    let response = session
        .post("https://account.nicovideo.jp/login/redirector", false)
        .form(&[("mail_tel", email_or_tel), ("password", password)])
        .send()
        .await?;

    let cookie_user_session = response
        .cookies()
        .find(|c| c.name() == "user_session" && c.value() != "deleted")
        .ok_or(Error::WrongLoginInfo)?;

    session.cookie_user_session = Some(cookie_user_session.value().to_owned());
    Ok(())
}
