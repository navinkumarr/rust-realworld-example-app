use db::schema::users;

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "users"]
#[primary_key(id)]
// Sequence of keys should be same as columns numbers
pub struct QueryUser {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct InsertUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
    pub updated_at: u64,
}

