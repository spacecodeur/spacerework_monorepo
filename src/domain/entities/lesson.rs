use super::user::User;

#[derive(Debug)]
pub struct Lesson {
    pub content: String,
    pub user: User,
}
