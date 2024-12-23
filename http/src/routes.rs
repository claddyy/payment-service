use actix_web::web;

use crate::features;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/healthcheck").service(features::healthcheck::controllers::healthcheck),
            )
            .service(
                web::scope("/user")
                    .service(features::user::controllers::register)
                    .service(features::user::controllers::login),
            )
            .service(
                web::scope("/transaction")
                    .service(features::transactions::controllers::create_transaction)
                    .service(features::transactions::controllers::get_transaction)
                    .service(features::transactions::controllers::list_user_transactions),
            )
            .service(
                web::scope("/account")
                    .service(features::accounts::controllers::get_account)
                    .service(features::accounts::controllers::get_balance)
                    .service(features::accounts::controllers::create_account)
                    .service(features::accounts::controllers::list_accounts),
            ),
    );
}
