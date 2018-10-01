use core::types::error::RepoError;
// -- Create article

#[derive(Debug, Serialize, Deserialize)]
pub struct NewArticle {
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}


#[derive(Debug, Deserialize)]
pub struct CreateArticleInput {
    pub user: NewArticle
}

#[derive(Debug, Serialize)]
pub struct CreateArticleOutput {
    pub user: NewArticle
}

pub trait ArticleRepo {
    fn save_new_article(&self, new_article: &NewArticle) -> Result<usize, RepoError>;
}
