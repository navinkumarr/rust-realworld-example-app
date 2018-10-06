use db::schema::articles;
use db::schema::tags;
use db::schema::article_tag;

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "articles"]
#[primary_key(id)]
// Sequence of keys should be same as columns numbers
pub struct QueryArticle {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub user_id: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Default, Insertable, Debug)]
#[table_name = "articles"]
pub struct InsertArticle<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub user_id: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "tags"]
#[primary_key(id)]
// Sequence of keys should be same as columns numbers
pub struct QueryTag {
    pub id: u32,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Default, Insertable, Debug)]
#[table_name = "tags"]
pub struct InsertTag {
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "article_tag"]
#[primary_key(article_id, tag_id)]
// Sequence of keys should be same as columns numbers
pub struct QueryArticleTag {
    pub article_id: u32,
    pub tag_id: u32,
}

#[derive(Default, Insertable, Debug)]
#[table_name = "article_tag"]
pub struct InsertArticleTag {
    pub article_id: u32,
    pub tag_id: u32,
}
