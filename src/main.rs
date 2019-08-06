#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate macros;

mod models;

use diesel::SqliteConnection;
use log::info;
use rocket::Rocket;
use rocket::http::RawStr;
use rocket::fairing::AdHoc;
use rocket_contrib::templates::{Template};
use models::{Blog, Post};

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("my_database")]
pub struct DbConn(SqliteConnection);


#[derive(Serialize)]
struct HelloContext {
    title: &'static str,
    name: Option<String>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[derive(Serialize)]
struct BlogsContext {
    title: &'static str,
    blog: Option<Blog>,
    blogs: Vec<Blog>,
    parent: &'static str
}

#[derive(Serialize)]
struct PostsContext {
    title: &'static str,
    post: Option<Post>,
    posts: Vec<Post>,
    parent: &'static str
}

#[get("/")]
fn index() -> Template {
    Template::render("hello", &HelloContext {
        title: "Home",
        name: Some(format!("Stranger")),
        parent: "layout"
    })
}

#[get("/<name>")]
fn hello(name: &RawStr) -> Template {
    Template::render("hello", &HelloContext {
        title: "Hello Someone",
        name: Some(format!("{}", name)),
        parent: "layout"
    })
}

#[get("/")]
fn list_blogs(conn: DbConn) -> Template {
    Template::render("layout", &BlogsContext{
        title: "Blog List",
        blogs: Blog::all(&conn),
        blog: None,
        parent: ""
    })
}

#[get("/<id>")]
fn get_blog(id: i32, conn: DbConn) -> Template {

    match Blog::get(id, &conn) {
        Ok(_blog) => Template::render("blogs", &BlogsContext {
            title: "A Blog",
            blog: Some(_blog),
            blogs: Blog::all(&conn),
            parent: "layout"
        }),
        Err(_err) => Template::render("blogs", &BlogsContext {
            title: "Blog not found.",
            blog: None,
            blogs: Blog::all(&conn),
            parent: "layout"
        })
    }
}


#[get("/")]
fn list_posts(conn: DbConn) -> Template {
    Template::render("layout", &PostsContext{
        title: "All User Posts",
        post: None,
        posts: Post::all(&conn),
        parent: "layout"
    })
}

#[get("/<id>")]
fn get_post(id: i32, conn: DbConn) -> Template {

    match Post::get(id, &conn) {
        Ok(_post) => Template::render("blogs", &PostsContext {
            title: "Viewing User Post",
            post: Some(_post),
            posts: Post::all(&conn),
            parent: "layout"
        }),
        Err(_err) => Template::render("blogs", &PostsContext {
            title: "Post not found.",
            post: None,
            posts: Post::all(&conn),
            parent: "layout"
        })
    }
}

/// This will be run when the server is started, running migrations if necessary.
fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => {
            info!("      => Migrations Ran OK");
            Ok(rocket)
        },
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}


fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", routes![index])
        .mount("/blogs", routes![list_blogs, get_blog])
        .mount("/posts", routes![list_posts, get_post])
        .mount("/hello", routes![hello])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

