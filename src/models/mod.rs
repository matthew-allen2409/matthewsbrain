use crate::handler::CommentInput;
use rss::Item;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Post {
    #[serde(default)]
    pub post_id: Option<i32>,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Post {
    pub fn to_rss_item(&self) -> Item {
        let id = self.post_id.unwrap().to_string();
        let mut item = Item::default();
        item.set_title(self.title.to_string());
        item.set_content(self.content.to_string());
        item.set_link(format!("https://matthewsbrain.com/post/{id}"));
        item.set_author("matthew.allen@matthewsbrain.com".to_string());
        item.set_pub_date(self.created_at.unwrap().to_string());

        item
    }
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Comment {
    pub post_id: i32,
    pub email: String,
    pub name: String,
    pub comment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Comment {
    pub fn from(input: CommentInput) -> Comment {
        Comment {
            post_id: input.post_id,
            email: input.email,
            name: input.name,
            comment: input.comment,
            created_at: chrono::Utc::now(),
        }
    }
}
