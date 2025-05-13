use cot::db::{Auto, ForeignKey, model};

#[model]
#[derive(Debug, Clone)]
pub struct Comment {
    #[model(primary_key)]
    id: Auto<i32>,
    parent: ForeignKey<Comment>,
    post_id: String,
    username: String,
    text: String,
}

impl Comment {
    #[expect(unused)]
    pub fn for_post(_post_id: &str) -> Vec<Self> {
        todo!()
    }
}
