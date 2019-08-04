#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate rocket_contrib;


use diesel::SqliteConnection;
use log::info;
use rocket::Rocket;
use rocket::http::RawStr;
use rocket::fairing::AdHoc;
use rocket_contrib::templates::{Template};

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("my_database")]
pub struct DbConn(SqliteConnection);



#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render("layout", &TemplateContext{
        title: "best title",
        name: Some(format!("Stranger")),
        items: vec![],
        parent: ""
    })
}

#[get("/<name>")]
fn hello(name: &RawStr) -> Template {
    Template::render("layout", &TemplateContext{
        title: "best title",
        name: Some(format!("{}", name)),
        items: vec![],
        parent: ""
    })
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
        .mount("/", routes![index, hello])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}


