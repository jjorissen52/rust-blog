use diesel::{self, prelude::*};
use diesel::result::Error;

//// blogs table
//mod schema {
//    table! {
//        blogs {
//            id -> Nullable<Integer>,
//            title -> Text,
//            text -> Text,
//        }
//    }
//}

mod schema;

use schema::{blogs, posts};
//use self::schema::blogs::dsl::{blogs as all_blogs, title as blog_title, text as blog_text};
use blogs::dsl::{blogs as all_blogs};
use posts::dsl::{posts as all_posts};

#[table_name="blogs"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: String,
    pub text: Option<String>,
}

#[table_name="posts"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, FromForm)]
pub struct Post {
    pub id: Option<i32>,
    pub username: String,
    pub text: Option<String>,
}

impl Blog {
    pub fn all(conn: &SqliteConnection) -> Vec<Blog> {
        all_blogs.order(blogs::id.desc()).load::<Blog>(conn).unwrap()
    }

    pub fn get(id: i32, conn: &SqliteConnection) -> Result<Blog, Error> {
        all_blogs.find(id).get_result::<Blog>(conn)
    }
}

impl Post {
    pub fn all(conn: &SqliteConnection) -> Vec<Post> {
        all_posts.order(posts::username.asc()).load::<Post>(conn).unwrap()
    }

    pub fn get(id: i32, conn: &SqliteConnection) -> Result<Post, Error> {
        all_posts.find(id).get_result::<Post>(conn)
    }
}