use askama::Template;

use crate::{db::paginate::Paginate, model::{Category, TopicArchive, TopicDetail, TopicList}};

#[derive(Template)]
#[template(path="frontend/topic_list.html")]
pub struct List {
    pub list: Paginate<Vec<TopicList>>,
    pub page : u32,
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
    pub category_name: String,
}

#[derive(Template)]
#[template(path="frontend/topic_detail.html")]
pub struct Detail {
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
    pub item: TopicDetail,
}

#[derive(Template)]
#[template(path="frontend/topic_arch.html")]
pub struct ArchiveList {
    pub list: Paginate<Vec<TopicList>>,
    pub page : u32,
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
    pub dt: String,
}