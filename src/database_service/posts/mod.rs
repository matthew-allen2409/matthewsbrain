use crate::models::Post;
use sqlx::mysql::MySqlPool;

pub async fn posts(pool: MySqlPool) -> Vec<Post> {
    sqlx::query_as::<_, Post>("Select * from posts")
        .fetch_all(&pool)
        .await
        .expect("Cannot fetch posts")
}

pub async fn get_post_by_id(pool: MySqlPool, id: u32) -> Post {
    sqlx::query_as::<_, Post>("Select * from posts where post_id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("Cannot fetch post by id")
}

pub async fn upload_post(pool: MySqlPool, post: &Post) {
    sqlx::query("INSERT INTO posts (title, content) values (?, ?)")
        .bind(&post.title)
        .bind(&post.content)
        .execute(&pool)
        .await
        .expect("Cannot insert post");
}

pub async fn delete_post(pool: MySqlPool, id: u32) {
    sqlx::query("DELETE FROM posts WHERE post_id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .expect("Cannot delete post");
}
