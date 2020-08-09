use super::{item::NicorepoItem, *};
use std::collections::VecDeque;

type FetchNicorepoFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(VecDeque<NicorepoItem>, bool)>> + 'a>>;

/// Streams nicorepo items. See also [nicorepo::stream()](super::stream).
pub struct NicorepoStream<'a> {
    session: &'a Session,
    content_filter: ContentFilter,
    sender_filter: SenderFilter,
    last_item_id: Option<String>,
    future: Option<FetchNicorepoFuture<'a>>,
    buf: VecDeque<NicorepoItem>,
    is_finished: bool,
}
impl<'a> Stream for NicorepoStream<'a> {
    type Item = Result<NicorepoItem>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        if self.buf.is_empty() && self.future.is_none() && !self.is_finished {
            self.future = Some(Box::pin(Self::gen_future(
                self.session,
                self.content_filter,
                self.sender_filter,
                self.last_item_id.take(),
            )));
        }
        if let Some(future) = self.future.as_mut() {
            match future.as_mut().poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(result) => {
                    self.future = None;
                    match result {
                        Ok((buf, is_finished)) => {
                            self.last_item_id = buf.iter().last().map(|i| i.id.clone());
                            self.buf = buf;
                            self.is_finished = is_finished;
                        }
                        Err(err) => return Poll::Ready(Some(Err(err))),
                    }
                }
            }
        }

        Poll::Ready(Ok(self.buf.pop_front()).transpose())
    }
}
impl<'a> NicorepoStream<'a> {
    pub fn new(
        session: &'a Session,
        content_filter: ContentFilter,
        sender_filter: SenderFilter,
    ) -> NicorepoStream {
        NicorepoStream {
            session,
            content_filter,
            sender_filter,
            last_item_id: None,
            future: None,
            buf: VecDeque::new(),
            is_finished: false,
        }
    }

    /// Returns `(items, is_finished)`
    async fn gen_future(
        session: &'a Session,
        content_filter: ContentFilter,
        sender_filter: SenderFilter,
        last_item_id: Option<String>,
    ) -> Result<(VecDeque<NicorepoItem>, bool)> {
        let url = gen_url(content_filter, sender_filter, last_item_id);
        let json = session.get_json(&url, None).await?;

        let status = json_extract!(json, as_u64, ["meta"]["status"]);
        match status {
            200 => (), //OK
            401 => bail!(Error::LoginRequired),
            _ => {
                let error_code = json_extract!(json, as_str, ["meta"]["errorCode"]);
                let error_message = json_extract!(json, as_str, ["meta"]["errorMessage"]);
                bail!(anyhow!("{} {}: {}", status, error_code, error_message)
                    .context(Error::InvalidResponse));
            }
        }

        let is_finished = !json_extract!(json, as_bool, ["meta"]["hasNext"]);

        let mut items = VecDeque::new();
        for item in json_extract!(json, as_array, ["data"]) {
            items.push_back(NicorepoItem::from_json(item)?);
        }

        Ok((items, is_finished))
    }
}

fn gen_url(
    content_filter: ContentFilter,
    sender_filter: SenderFilter,
    last_item_id: Option<String>,
) -> String {
    let mut params: Vec<(&str, &str)> = vec![];

    if let Some(last_item_id) = &last_item_id {
        params.push(("untilId", last_item_id));
    }

    match sender_filter {
        SenderFilter::All => (),
        SenderFilter::LoginUser => params.push(("list", "self")),
        SenderFilter::FollowingUsers => params.push(("list", "followingUser")),
        SenderFilter::FollowingChannels => params.push(("list", "followingChannel")),
        SenderFilter::FollowingCommunities => params.push(("list", "followingCommunity")),
        SenderFilter::FollowingMylists => params.push(("list", "followingMylist")),
    }
    match content_filter {
        ContentFilter::All => (),
        ContentFilter::VideoUploads => {
            params.push(("object[type]", "video"));
            params.push(("type", "upload"));
        }
        ContentFilter::LivePrograms => {
            params.push(("object[type]", "video"));
            params.push(("type", "upload"));
        }
        ContentFilter::IllustUploads => {
            params.push(("object[type]", "image"));
            params.push(("type", "add"));
        }
        ContentFilter::ComicUploads => {
            params.push(("object[type]", "comicStory"));
            params.push(("type", "add"));
        }
        ContentFilter::ArticleUploads => {
            params.push(("object[type]", "article"));
            params.push(("type", "add"));
        }
        ContentFilter::GameUploads => {
            params.push(("object[type]", "game"));
            params.push(("type", "add"));
        }
    }

    let mut url =
        "https://public.api.nicovideo.jp/v1/timelines/nicorepo/last-1-month/my/pc/entries.json"
            .to_owned();
    if !params.is_empty() {
        url += "?";
    }
    url += &params
        .iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<_>>()
        .join("&");
    url
}
