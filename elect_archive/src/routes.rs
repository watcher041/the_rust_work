
use actix_web::web;
use crate::controllers::*;

pub fn routing(cfg: &mut web::ServiceConfig) {

    // ホーム画面
    cfg.route("/", web::get().to(user_controller::index));

    // ユーザー関連画面
    cfg.service(
        web::scope("/user")
            .route( "" ,  web::get().to(user_controller::index) )
    );

}