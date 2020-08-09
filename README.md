# niconico-rs
A Rust wrapper around (unpublished) niconico API

# Features

- Log in to niconico
```rust
let mut session = Session::new(USER_AGENT, Language::Japanese);
session.login(EMAIL_OR_TEL, PASSWORD).await?;
```

- Fetch user details
```rust
let login_user_details = User::LoginUser.fetch_details(&session).await?;
let user_1_details = User::UserId(1).fetch_details(&session).await?;
```

- Stream nicorepo
```rust
let mut nicorepo_stream = nicorepo::stream(&session, ContentFilter::All, SenderFilter::All);
while let Some(item) = nicorepo_stream.next().await {
    println!("{:#?}", item);
}
```