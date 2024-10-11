use crate::models::Post;
use sqlx::mysql::{MySqlPool, MySqlQueryResult};

pub async fn posts(pool: MySqlPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("Select * from posts")
        .fetch_all(&pool)
        .await
}

pub async fn get_post_by_id(pool: &MySqlPool, id: i32) -> Result<Post, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE post_id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn upload_post(pool: MySqlPool, post: &Post) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO posts (title, content) values (?, ?)")
        .bind(&post.title)
        .bind(&post.content)
        .execute(&pool)
        .await
}

pub async fn delete_post(pool: MySqlPool, id: u32) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM posts WHERE post_id = ?")
        .bind(id)
        .execute(&pool)
        .await
}
