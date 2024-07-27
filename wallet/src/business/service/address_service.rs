// // 用于 Actix-web 路由
// pub async fn create_address_handler(pool: web::Data<PgPool>, item: web::Json<Address>) -> HttpResponse {
//     match item.create(&pool).await {
//         Ok(address) => HttpResponse::Created().json(address),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn get_address_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Address::get(&pool, guid).await {
//         Ok(Some(address)) => HttpResponse::Ok().json(address),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn update_address_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>, item: web::Json<Address>) -> HttpResponse {
//     match item.update(&pool, guid).await {
//         Ok(Some(address)) => HttpResponse::Ok().json(address),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn delete_address_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Address::delete(&pool, guid).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn list_addresses_handler(pool: web::Data<PgPool>) -> HttpResponse {
//     match Address::list(&pool).await {
//         Ok(addresses) => HttpResponse::Ok().json(addresses),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }