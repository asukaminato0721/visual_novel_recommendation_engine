use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VnTitle {
    pub vn_id: i32,
    pub language: Arc<str>,
    pub official: bool,
    pub title: Arc<str>,
    pub latin_title: Option<Arc<str>>,
}

#[derive(Debug, Clone)]
pub struct Rating {
    pub vn_id: i32,
    pub user_id: i32,
    pub rating: f64,
    pub date: Arc<str>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub tag_id: i32,
    pub vn_id: i32,
    pub rating: f64,
}
