use db::models::article::*;
use core::types::article::*;
use core::types::user::CurrentUser;
use diesel::prelude::*;
use db::pool::*;
use core::types::error::RepoError;
use db::schema::*;
use diesel;
use core::utils;

pub struct MysqlArticleRepo<'a> {
    db_conn: &'a DbConn,
}

impl<'a> MysqlArticleRepo<'a> {
    pub fn new(conn: &'a DbConn) -> MysqlArticleRepo<'a> {
        MysqlArticleRepo { db_conn: conn }
    }
}

impl<'a> ArticleRepo for MysqlArticleRepo<'a> {
    fn save_new_article(
        &self,
        new_article: &NewArticle,
        current_user: &CurrentUser,
    ) -> Result<Article, RepoError> {
        let date_time = utils::YmdHMS();
        let slug = new_article.slug.as_ref().unwrap().clone();

        let data_articles = InsertArticle {
            title: &new_article.title,
            slug: &slug,
            description: &new_article.description,
            body: &new_article.body,
            user_id: current_user.id,
            created_at: date_time,
            updated_at: date_time,
        };

        debug!("data_articles {:?}", data_articles);

        diesel::insert_into(articles::table)
            .values(&data_articles)
            .execute(&*self.db_conn.master)?;

        // Need a better way to get last insert id
        // https://github.com/diesel-rs/diesel/issues/1011
        let query_articles_id = articles::table
            .filter(articles::user_id.eq(&current_user.id))
            .filter(articles::slug.eq(&slug))
            .select(articles::id)
            .first::<u32>(&*self.db_conn.master)?;

        debug!("query_articles_id {}", query_articles_id);

        for name in &new_article.tag_list{
            let insert_tag = InsertTag {
                name: name.clone(),
                created_at: date_time,
                updated_at: date_time,
            };

            diesel::insert_into(tags::table)
                .values(&insert_tag)
                .execute(&*self.db_conn.master)?;

            let query_tag_id = tags::table
                .filter(tags::name.eq(&name))
                .select(tags::id)
                .first::<u32>(&*self.db_conn.master)?;

            let insert_article_tag = InsertArticleTag{
                article_id: query_articles_id,
                tag_id: query_tag_id
            };

            diesel::insert_into(article_tag::table)
                .values(&insert_article_tag)
                .execute(&*self.db_conn.master)?;
        };

        let result_articles = Article {
            slug: slug.clone(),
            title: new_article.title.clone(),
            description: new_article.description.clone(),
            body: new_article.body.clone(),
            user_id: current_user.id,
            tag_list: new_article.tag_list.clone(),
            created_at: date_time,
            updated_at: date_time,
            ..Article::default()
        };

        Ok(result_articles)
    }
}
