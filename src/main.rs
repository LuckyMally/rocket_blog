#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
use std::vec;
use glob::glob;

struct Article {
    title: String,
    front_image_path: String,
    document_path: String
}
impl Article {
    fn new() -> Article {
        Article{
            title: "".to_string(),
            front_image_path: "".to_string(),
            document_path: "".to_string()
        }
    }
}
fn main() {
    let mut article_store: Vec<Article> = Vec::new();
    for entry in glob("Assets/*").expect("Failed to read glob pattern") {

        let mut article: Article = Article::new();
        let x = entry.unwrap();
        if x.metadata().unwrap().is_dir(){
            article.title = x.display().to_string();
            
            let find_folder = format!("{}/*.jpg", x.display().to_string());
            for entry in glob(find_folder.as_str()).expect("Failed to read glob pattern") {
                article.front_image_path = entry.unwrap().display().to_string();
            }
            
            let find_folder = format!("{}/*.html", x.display().to_string());
            for entry in glob(find_folder.as_str()).expect("Failed to read glob pattern") {
                article.document_path = entry.unwrap().display().to_string();
            }
            article_store.push(article);
        }
    }

    for a in article_store{
        println!("{},{},{}",a.title, a.document_path, a.front_image_path);
    }

    rocket::ignite().mount("/", routes![index]).launch();
}