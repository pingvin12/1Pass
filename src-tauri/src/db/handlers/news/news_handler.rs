use std::error::Error;

use diesel::query_builder::AsQuery;
use diesel::{insert_into, RunQueryDsl, QueryDsl, QueryResult};

use crate::db::database::Database;
use crate::db::domain::news::news_object::{News, NewNews};
use crate::schema::news::dsl::*;
use crate::diesel::ExpressionMethods;
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
            .execute(&self.connection)
            .expect("Error while creating new news");
        Ok(())
    }

    fn query_news(&mut self) -> Result<Vec<News>, Box<dyn Error>> {
        Ok(news.order_by(title.asc()).filter(published.eq(true)).get_results(&mut self.connection)?)
    }

    fn delete_news(&mut self, news_id: &i32) -> Result<(), Box<dyn Error>> {
        diesel::delete(news.filter(id.eq(news_id)))
            .execute(&self.connection)
            .expect("Error while deleting news");
        Ok(())
    }
}