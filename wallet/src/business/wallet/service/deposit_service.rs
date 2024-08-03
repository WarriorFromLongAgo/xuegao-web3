//
//
// // 用于 Actix-web 路由
// pub async fn create_deposit_handler(pool: web::Data<PgPool>, item: web::Json<Deposit>) -> HttpResponse {
//     match item.create(&pool).await {
//         Ok(deposit) => HttpResponse::Created().json(deposit),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn get_deposit_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Deposit::get(&pool, guid).await {
//         Ok(Some(deposit)) => HttpResponse::Ok().json(deposit),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn update_deposit_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>, item: web::Json<Deposit>) -> HttpResponse {
//     match item.update(&pool, guid).await {
//         Ok(Some(deposit)) => HttpResponse::Ok().json(deposit),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn delete_deposit_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Deposit::delete(&pool, guid).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn list_deposits_handler(pool: web::Data<PgPool>) -> HttpResponse {
//     match Deposit::list(&pool).await {
//         Ok(deposits) => HttpResponse::Ok().json(deposits),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
