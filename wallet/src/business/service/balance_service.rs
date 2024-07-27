//
//
// // 用于 Actix-web 路由
// pub async fn create_balance_handler(pool: web::Data<PgPool>, item: web::Json<Balance>) -> HttpResponse {
//     match item.create(&pool).await {
//         Ok(balance) => HttpResponse::Created().json(balance),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn get_balance_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Balance::get(&pool, guid).await {
//         Ok(Some(balance)) => HttpResponse::Ok().json(balance),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn update_balance_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>, item: web::Json<Balance>) -> HttpResponse {
//     match item.update(&pool, guid).await {
//         Ok(Some(balance)) => HttpResponse::Ok().json(balance),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn delete_balance_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Balance::delete(&pool, guid).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn list_balances_handler(pool: web::Data<PgPool>) -> HttpResponse {
//     match Balance::list(&pool).await {
//         Ok(balances) => HttpResponse::Ok().json(balances),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }