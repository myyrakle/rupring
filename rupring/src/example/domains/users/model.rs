pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub type Users = Vec<User>;
