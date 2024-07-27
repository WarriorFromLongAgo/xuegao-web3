//
// // 用于 Actix-web 路由
// pub async fn create_transaction_handler(pool: web::Data<PgPool>, item: web::Json<Transaction>) -> HttpResponse {
//     match item.create(&pool).await {
//         Ok(transaction) => HttpResponse::Created().json(transaction),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn get_transaction_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Transaction::get(&pool, guid).await {
//         Ok(Some(transaction)) => HttpResponse::Ok().json(transaction),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn update_transaction_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>, item: web::Json<Transaction>) -> HttpResponse {
//     match item.update(&pool, guid).await {
//         Ok(Some(transaction)) => HttpResponse::Ok().json(transaction),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn delete_transaction_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Transaction::delete(&pool, guid).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn list_transactions_handler(pool: web::Data<PgPool>) -> HttpResponse {
//     match Transaction::list(&pool).await {
//         Ok(transactions) => HttpResponse::Ok().json(transactions),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }