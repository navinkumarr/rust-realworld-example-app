use db::schema::users;

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "users"]
#[primary_key(id)]
pub struct QueryUser {
    pub id: u32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: String,
    pub token: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct InsertUser {
    pub email: String,
    pub username: String,
    pub token: String,
}
