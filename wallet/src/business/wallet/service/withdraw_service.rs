//
// // 用于 Actix-web 路由
// pub async fn create_withdraw_handler(pool: web::Data<PgPool>, item: web::Json<Withdraw>) -> HttpResponse {
//     match item.create(&pool).await {
//         Ok(withdraw) => HttpResponse::Created().json(withdraw),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn get_withdraw_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Withdraw::get(&pool, guid).await {
//         Ok(Some(withdraw)) => HttpResponse::Ok().json(withdraw),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn update_withdraw_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>, item: web::Json<Withdraw>) -> HttpResponse {
//     match item.update(&pool, guid).await {
//         Ok(Some(withdraw)) => HttpResponse::Ok().json(withdraw),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn delete_withdraw_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Withdraw::delete(&pool, guid).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn list_withdraws_handler(pool: web::Data<PgPool>) -> HttpResponse {
//     match Withdraw::list(&pool).await {
//         Ok(withdraws) => HttpResponse::Ok().json(withdraws),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }