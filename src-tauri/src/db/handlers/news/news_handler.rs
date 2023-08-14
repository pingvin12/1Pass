use std::error::Error;

use diesel::query_builder::AsQuery;
use diesel::{insert_into, QueryDsl, QueryResult, RunQueryDsl};

use crate::db::database::Database;
use crate::db::domain::news::news_object::{NewNews, News};
use crate::diesel::ExpressionMethods;
use crate::schema::news::dsl::*;
pub trait NewsManager {
    fn create_news(
        &mut self,
        inserted_title: &str,
        inserted_content: &str,
    ) -> Result<(), Box<dyn Error>>;
    fn delete_news(&mut self, news_id: &i32) -> Result<(), Box<dyn Error>>;
    fn query_news(&mut self) -> Result<Vec<News>, Box<dyn Error>>;
}

impl NewsManager for Database {
    fn create_news(
        &mut self,
        inserted_title: &str,
        inserted_content: &str,
    ) -> Result<(), Box<dyn Error>> {
        use crate::schema::news::dsl::*;
        let new_news = NewNews {
            title: inserted_title,
            content: inserted_content,
        };

        insert_into(news)
            .values(&new_news)
            .execute(&mut *self.conn)
            .expect("Error while creating new news");
        Ok(())
    }

    fn query_news(&mut self) -> Result<Vec<News>, Box<dyn Error>> {
        Ok(news
            .order_by(title.asc())
            .filter(published.eq(true))
            .get_results(&mut *self.conn)?)
    }

    fn delete_news(&mut self, news_id: &i32) -> Result<(), Box<dyn Error>> {
        diesel::delete(news.filter(id.eq(news_id)))
            .execute(&mut *self.conn)
            .expect("Error while deleting news");
        Ok(())
    }
}
