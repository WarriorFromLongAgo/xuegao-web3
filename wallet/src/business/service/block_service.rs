//
//
//
// // 用于 Actix-web 路由
// pub async fn create_block_handler(pool: web::Data<PgPool>, item: web::Json<Block>) -> HttpResponse {
//     match item.create(&pool).await {
//         Ok(block) => HttpResponse::Created().json(block),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn get_block_handler(pool: web::Data<PgPool>, web::Path(hash): web::Path<String>) -> HttpResponse {
//     match Block::get(&pool, &hash).await {
//         Ok(Some(block)) => HttpResponse::Ok().json(block),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn update_block_handler(pool: web::Data<PgPool>, web::Path(hash): web::Path<String>, item: web::Json<Block>) -> HttpResponse {
//     match item.update(&pool, &hash).await {
//         Ok(Some(block)) => HttpResponse::Ok().json(block),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn delete_block_handler(pool: web::Data<PgPool>, web::Path(hash): web::Path<String>) -> HttpResponse {
//     match Block::delete(&pool, &hash).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// pub async fn list_blocks_handler(pool: web::Data<PgPool>) -> HttpResponse {
//     match Block::list(&pool).await {
//         Ok(blocks) => HttpResponse::Ok().json(blocks),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }