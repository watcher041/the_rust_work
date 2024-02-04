
use actix_web::{Responder,HttpResponse};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "users/index.stpl")]
struct HelloTemplate {
    messages: Vec<String>
}

pub async fn index() -> impl Responder {
    
    let vars = HelloTemplate {
        messages: 
            vec![String::from("foo"), 
                String::from("bar")],
    };
    let html = vars.render_once().unwrap();
    
    HttpResponse::Ok().body(html)
}