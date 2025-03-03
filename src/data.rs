use std::sync::Arc;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VnTitle {
    pub vn_id: i32,
    pub language: Arc<str>,
    pub official: bool,
    pub title: Arc<str>,
    pub latin_title: Option<Arc<str>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Rating {
    pub vn_id: i32,
    pub user_id: i32,
    pub rating: f64,
    pub date: Arc<str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub tag_id: i32,
    pub vn_id: i32,
    pub rating: f64,
}
