pub fn get_users() -> &'static str{
    "Hello, Users!"
}

pub fn get_user_by_id(id: u32) -> String {
    format!("Hello, User {}!", id)
}
