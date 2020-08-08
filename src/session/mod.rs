use crate::*;

mod login;

#[derive(Debug, Clone)]
pub struct Session {
    client: reqwest::Client,

    cookie_user_session: Option<String>,
    language: Language,
}
impl Session {
    pub fn new<'a, T>(user_agent: T, language: Language) -> Session
    where
        T: Into<Option<&'a str>>,
    {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept-Language", language.into_header_value());

        Session {
            client: reqwest::ClientBuilder::new()
                .user_agent(user_agent.into().unwrap_or_default())
                .redirect(reqwest::redirect::Policy::none())
                .default_headers(headers)
                .referer(false)
                .build()
                .unwrap(),
            cookie_user_session: None,
            language,
        }
    }
    pub async fn login(&mut self, email_or_tel: &str, password: &str) -> Result<()> {
        login::login(self, email_or_tel, password).await
    }
    pub fn get_cookie_user_session(&self) -> Option<&str> {
        self.cookie_user_session.as_deref()
    }
    pub fn set_cookie_user_session(&mut self, cookie_user_session: impl Into<Cow<'static, str>>) {
        self.cookie_user_session = Some(cookie_user_session.into().into_owned());
    }
    pub fn is_logged_in(&self) -> bool {
        self.cookie_user_session.is_some()
    }

    pub(crate) fn get(&self, url: &str, include_cookie: bool) -> reqwest::RequestBuilder {
        let mut req = self.client.get(url);
        if include_cookie && self.cookie_user_session.is_some() {
            req = req.header(
                "Cookie",
                &format!(
                    "user_session={}",
                    self.cookie_user_session.as_ref().unwrap()
                ),
            );
        }
        req
    }
    pub(crate) async fn get_data<T>(&self, url: &str, include_cookie: bool) -> Result<T>
    where
        T: html_extractor::HtmlExtractor,
    {
        let html_str = self.get(url, include_cookie).send().await?.text().await?;
        let data = html_extractor::HtmlExtractor::extract_from_str(&html_str)?;
        Ok(data)
    }
    pub(crate) fn post(&self, url: &str, include_cookie: bool) -> reqwest::RequestBuilder {
        let mut req = self.client.post(url);
        if include_cookie && self.cookie_user_session.is_some() {
            req = req.header(
                "Cookie",
                &format!(
                    "user_session={}",
                    self.cookie_user_session.as_ref().unwrap()
                ),
            );
        }
        req
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Language {
    Japanese,
    English,
    Chinese,
}
impl Language {
    pub fn into_header_value(self) -> reqwest::header::HeaderValue {
        match self {
            Language::Japanese => reqwest::header::HeaderValue::from_static("ja"),
            Language::English => reqwest::header::HeaderValue::from_static("en"),
            Language::Chinese => reqwest::header::HeaderValue::from_static("zh"),
        }
    }
}
