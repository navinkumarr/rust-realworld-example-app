use core::types::article::*;
use core::types::user::*;
use core::types::error::*;

pub fn create_article<U, A>(
    current_user: CurrentUser,
    create_article_input: CreateArticleInput,
    user_repo: &U,
    article_repo: &A
) -> Result<CreateArticleOutput, CreateArticleError>
where 
    U: UserRepo,
    A: ArticleRepo
{
    let slug = Some(create_article_input.article.slug.unwrap_or(::core::utils::slug(create_article_input.article.title.as_str())));
    let new_article: NewArticle = NewArticle { slug: slug , ..create_article_input.article };
    let article = article_repo.save_new_article(&new_article, &current_user)?;
    let user = user_repo.find_user_by_username(&current_user.username)?;

    //Since user is logged in we can unwrap
    let article_with_author = ArticleWithAuthor::from((article, user.unwrap()));

    debug!("{:?}", article_with_author);
    
    Ok(CreateArticleOutput { article: article_with_author })
}
