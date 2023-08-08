use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::{Context, Tera}; // new line

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsong error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[get("/")]
async fn home(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Home");
    let template = tera.render("pages/home.html", &context).expect("Error");
    HttpResponse::Ok().body(template)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .service(
                fs::Files::new("/static", "static/")
                    .show_files_listing()
                    .use_last_modified(true),
            ) // new line
            .service(home)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
