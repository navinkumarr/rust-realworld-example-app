use core::types::error::RepoError;
use core::types::user::{User, CurrentUser};

// -- Create article
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub user_id: u32,
    pub tag_list: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub favorites_count: u32,
    pub favorited: bool,
}

impl From<(Article, User)> for ArticleWithAuthor {
    fn from((article, user): (Article, User)) -> ArticleWithAuthor {
        ArticleWithAuthor {
            slug : article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: article.tag_list,
            created_at: article.created_at,
            updated_at: article.updated_at,
            favorites_count: article.favorites_count,
            favorited: article.favorited,
            author: user 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub description: String,
    pub body: String,
    pub slug: Option<String>,
    pub tag_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleWithAuthor {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub favorites_count: u32,
    pub favorited: bool,
    pub author: User,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleInput {
    pub article: NewArticle
}

#[derive(Debug, Serialize)]
pub struct CreateArticleOutput {
    pub article: ArticleWithAuthor
}

pub trait ArticleRepo {
    fn save_new_article(&self, new_article: &NewArticle, current_user: &CurrentUser) -> Result<Article, RepoError>;
}
