use crate::*;

mod login;

/// A session in which all requests are made.
#[derive(Debug, Clone)]
pub struct Session {
    client: reqwest::Client,

    /// The value of cookie `user_session` obtained on login.
    cookie_user_session: Option<String>,
    /// The language to include in every request as `Accept-Language`.
    language: Language,
}
impl Session {
    /// Creates a new session. `user_agent` should be the name of the application.
    ///
    /// # Panics
    /// This method panics if it cannot create a HTTP client.
    ///
    /// # Examples
    /// ```
    /// # use niconico::*;
    /// const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    /// let session = Session::new(USER_AGENT, Language::Japanese);
    /// ```
    pub fn new<'a, T>(user_agent: T, language: Language) -> Session
    where
        T: Into<Option<&'a str>>,
    {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept-Language", language.into());

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
    /// Log in to niconico using specified email address or telephone number and password.
    ///
    /// # Errors
    /// This method returns `Error::WrongLoginInfo` if the given information is wrong.
    ///
    /// # Examples
    /// ```no_run
    /// # use niconico::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    /// # let mut session = Session::new(USER_AGENT, Language::Japanese);
    /// const EMAIL_OR_TEL: &str = env!("NICO_EMAIL");
    /// const PASSWORD: &str = env!("NICO_PASS");
    /// session.login(EMAIL_OR_TEL, PASSWORD).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login(&mut self, email_or_tel: &str, password: &str) -> Result<()> {
        login::login(self, email_or_tel, password).await
    }
    /// Gets the value of cookie `user_session` if already logged in.
    /// The value can be used to keep logged in for some days.
    /// To restore the session, use [`set_cookie_user_session`](Session::set_cookie_user_session).
    ///
    /// # Examples
    /// ```
    /// # use niconico::*;
    /// # const EMAIL_OR_TEL: &str = env!("NICO_EMAIL");
    /// # const PASSWORD: &str = env!("NICO_PASS");
    /// # const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut session = Session::new(USER_AGENT, Language::Japanese);
    /// assert_eq!(session.get_cookie_user_session(), None);
    ///
    /// session.login(EMAIL_OR_TEL, PASSWORD).await?;
    /// let cookie = session.get_cookie_user_session().unwrap();
    ///
    /// let mut new_session = Session::new(USER_AGENT, Language::Japanese);
    /// new_session.set_cookie_user_session(cookie);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_cookie_user_session(&self) -> Option<&str> {
        self.cookie_user_session.as_deref()
    }
    /// Sets the value of cookie `user_session`. This method does not check the validity of the cookie.
    /// See [`get_cookie_user_session`](Session::get_cookie_user_session) for examples.
    pub fn set_cookie_user_session(&mut self, cookie_user_session: &str) {
        self.cookie_user_session = Some(cookie_user_session.to_owned());
    }
    /// Returns whether logged in or not.
    pub fn is_logged_in(&self) -> bool {
        self.cookie_user_session.is_some()
    }

    /// Makes a GET request. Includes cookie `user_session` if `include_cookie` is `true`.
    pub(crate) fn get(
        &self,
        url: &str,
        options: impl Into<Option<RequestOptions>>,
    ) -> reqwest::RequestBuilder {
        let options = options.into().unwrap_or_default();

        let mut req = self.client.get(url);
        if options.cookie_user_session && self.cookie_user_session.is_some() {
            req = req.header(
                "Cookie",
                &format!(
                    "user_session={}",
                    self.cookie_user_session.as_ref().unwrap()
                ),
            );
        }
        if options.header_x_frontend_id {
            req = req.header("X-Frontend-Id", 6);
        }
        req
    }
    /// Gets html and extracts data from it.
    pub(crate) async fn get_data<T>(
        &self,
        url: &str,
        options: impl Into<Option<RequestOptions>>,
    ) -> Result<T>
    where
        T: html_extractor::HtmlExtractor,
    {
        let html_str = self
            .get(url, options)
            .send()
            .await
            .with_context(|| format!("cannot fetch from `{}`", url))
            .context(Error::InvalidResponse)?
            .text()
            .await
            .with_context(|| format!("cannot fetch from `{}`", url))
            .context(Error::InvalidResponse)?;
        let data = html_extractor::HtmlExtractor::extract_from_str(&html_str)
            .context(Error::InvalidResponse)?;
        Ok(data)
    }
    /// Gets json and parses it.
    pub(crate) async fn get_json(
        &self,
        url: &str,
        options: impl Into<Option<RequestOptions>>,
    ) -> Result<serde_json::Value> {
        let json_str = self
            .get(url, options)
            .send()
            .await
            .with_context(|| format!("cannot fetch from `{}`", url))
            .context(Error::InvalidResponse)?
            .text()
            .await
            .with_context(|| format!("cannot fetch from `{}`", url))
            .context(Error::InvalidResponse)?;
        let json = serde_json::from_str(&json_str).context(Error::InvalidResponse)?;
        Ok(json)
    }
    /// Makes a POST request. Includes cookie `user_session` if `include_cookie` is `true`.
    pub(crate) fn post(
        &self,
        url: &str,
        options: impl Into<Option<RequestOptions>>,
    ) -> reqwest::RequestBuilder {
        let options = options.into().unwrap_or_default();

        let mut req = self.client.post(url);
        if options.cookie_user_session && self.cookie_user_session.is_some() {
            req = req.header(
                "Cookie",
                &format!(
                    "user_session={}",
                    self.cookie_user_session.as_ref().unwrap()
                ),
            );
        }
        if options.header_x_frontend_id {
            req = req.header("X-Frontend-Id", 6);
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
impl Into<reqwest::header::HeaderValue> for Language {
    fn into(self) -> reqwest::header::HeaderValue {
        match self {
            Language::Japanese => reqwest::header::HeaderValue::from_static("ja"),
            Language::English => reqwest::header::HeaderValue::from_static("en"),
            Language::Chinese => reqwest::header::HeaderValue::from_static("zh"),
        }
    }
}

pub(crate) struct RequestOptions {
    /// Whether cookie `user_session` is included in requests.
    pub cookie_user_session: bool,
    /// Whether header `X-Frontend-Id` is include in requests.
    pub header_x_frontend_id: bool,
}
impl Default for RequestOptions {
    fn default() -> Self {
        RequestOptions {
            cookie_user_session: true,
            header_x_frontend_id: false,
        }
    }
}
