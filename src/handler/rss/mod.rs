use rss::ChannelBuilder;
use axum::response::Response;
use axum::extract::State;
use sqlx::mysql::MySqlPool;
use crate::database_service::posts::posts;

pub async fn handle_rss_feed(
    State(pool): State<MySqlPool>,
) -> Response<String> {
    let posts = match posts(pool).await {
        Ok(posts) => posts,
        Err(_) => return Response::builder().status(axum::http::StatusCode::NOT_FOUND).body(String::new()).unwrap(),
    };

    let channel: rss::Channel = ChannelBuilder::default()
        .title(String::from("Matthew's Brain!"))
        .link(String::from("https://matthewsbrain.com"))
        .description(String::from("My thoughts online!"))
        .items(posts.iter().map(|post| post.to_rss_item()).collect::<Vec<rss::Item>>())
        .build();

    Response::builder()
        .header("Content-Type", "text/xml")
        .body(channel.to_string())
        .unwrap()
}
