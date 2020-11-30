extern crate hacking_news_domain;
extern crate hacking_news_infra;

use hacking_news_domain::News;

pub async fn get_news_by_id(id: &String) -> Option<News> {
    if id.is_empty() {
        return None
    }
    return hacking_news_infra::get_news_by_id(id).await;
}

pub async fn delete_news_by_id(id: &String) -> Option<bool> {
    if id.is_empty() {
        return None
    }
    return hacking_news_infra::delete_news_by_id(id).await;
 }

 pub async fn list_news() -> Option<Vec<News>> {
    return hacking_news_infra::list_news().await;
 }

 pub async fn insert_news(url: &String, desc: &String) -> Option<News> {
    if url.is_empty() || desc.is_empty() {
        return None
    }
    return hacking_news_infra::insert_news(&url, &desc).await;
 }
