use diesel::{self, prelude::*};
use diesel::result::Error;

// blogs table
mod schema {
    table! {
        blogs {
            id -> Nullable<Integer>,
            title -> Text,
            text -> Text,
        }
    }
}

use self::schema::blogs;
//use self::schema::blogs::dsl::{blogs as all_blogs, title as blog_title, text as blog_text};
use self::schema::blogs::dsl::{blogs as all_blogs};

#[table_name="blogs"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: String,
    pub text: String
}

#[derive(FromForm)]
pub struct Post {
    pub title: String,
    pub content: String
}

impl Blog {
    pub fn all(conn: &SqliteConnection) -> Vec<Blog> {
        all_blogs.order(blogs::id.desc()).load::<Blog>(conn).unwrap()
    }

    pub fn get(id: i32, conn: &SqliteConnection) -> Result<Blog, Error> {
        all_blogs.find(id).get_result::<Blog>(conn)
    }
}