// // 用于 Actix-web 路由
// pub async fn create_token_handler(pool: web::Data<PgPool>, item: web::Json<Token>) -> HttpResponse {
//     match item.create(&pool).await {
//         Ok(token) => HttpResponse::Created().json(token),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn get_token_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Token::get(&pool, guid).await {
//         Ok(Some(token)) => HttpResponse::Ok().json(token),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn update_token_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>, item: web::Json<Token>) -> HttpResponse {
//     match item.update(&pool, guid).await {
//         Ok(Some(token)) => HttpResponse::Ok().json(token),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn delete_token_handler(pool: web::Data<PgPool>, web::Path(guid): web::Path<i32>) -> HttpResponse {
//     match Token::delete(&pool, guid).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn list_tokens_handler(pool: web::Data<PgPool>) -> HttpResponse {
//     match Token::list(&pool).await {
//         Ok(tokens) => HttpResponse::Ok().json(tokens),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
