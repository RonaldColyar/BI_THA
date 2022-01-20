use std::collections::HashSet;

pub struct User {
    pub screen_name: String,
}

pub struct Room {
    pub user_ids: HashSet<usize>,
}
