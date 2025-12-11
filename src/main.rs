use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use sqlx::PgPool;
use dotenvy::dotenv;

// Estruturas com documentação automática
#[derive(Serialize, Deserialize, Clone, ToSchema, sqlx::FromRow)]
pub struct Item {
    id: i32,
    nome: String,
    preco: f64,
}

#[derive(Deserialize, ToSchema)]
pub struct NovoItem {
    nome: String,
    preco: f64,
}

#[derive(Deserialize, ToSchema, Default)]
pub struct ListarParams {
    #[serde(default)]
    busca: Option<String>,
    #[serde(default)]
    ordenar_por: Option<String>,
    #[serde(default)]
    ordem: Option<String>,
    #[serde(default)]
    pagina: Option<i64>,
    #[serde(default)]
    por_pagina: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct ListarResponse {
    itens: Vec<Item>,
    total: i64,
    pagina: i64,
    por_pagina: i64,
    total_paginas: i64,
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

// Handlers
async fn raiz() -> &'static str {
    "Bem-vindo à API Rust! Acesse /swagger-ui para documentação."
}

#[utoipa::path(
    get,
    path = "/itens",
    params(
        ("busca" = Option<String>, Query, description = "Busca por ID ou nome do produto"),
        ("ordenar_por" = Option<String>, Query, description = "Campo para ordenação: id, nome, preco"),
        ("ordem" = Option<String>, Query, description = "Direção: asc ou desc"),
        ("pagina" = Option<i64>, Query, description = "Número da página (começa em 1)"),
        ("por_pagina" = Option<i64>, Query, description = "Itens por página (padrão: 10)")
    ),
    responses(
        (status = 200, description = "Lista de itens paginada", body = ListarResponse)
    )
)]
async fn listar_itens(
    State(estado): State<AppState>,
    Query(params): Query<ListarParams>,
) -> Result<Json<ListarResponse>, StatusCode> {
    let por_pagina = params.por_pagina.unwrap_or(10).max(1).min(100);
    let pagina = params.pagina.unwrap_or(1).max(1);
    let offset = (pagina - 1) * por_pagina;

    let ordem_col = match params.ordenar_por.as_deref() {
        Some("nome") => "nome",
        Some("preco") => "preco",
        _ => "id",
    };
    let ordem_dir = match params.ordem.as_deref() {
        Some("desc") => "DESC",
        _ => "ASC",
    };

    let (itens, total): (Vec<Item>, i64) = if let Some(busca) = &params.busca {
        if let Ok(id) = busca.parse::<i32>() {
            let query = format!(
                "SELECT id, nome, preco FROM itens WHERE id = $1 OR nome ILIKE $2 ORDER BY {} {} LIMIT $3 OFFSET $4",
                ordem_col, ordem_dir
            );
            let count_query = "SELECT COUNT(*) FROM itens WHERE id = $1 OR nome ILIKE $2";
            
            let pattern = format!("%{}%", busca);
            let itens = sqlx::query_as::<_, Item>(&query)
                .bind(id)
                .bind(&pattern)
                .bind(por_pagina)
                .bind(offset)
                .fetch_all(&estado.pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let total: (i64,) = sqlx::query_as(count_query)
                .bind(id)
                .bind(&pattern)
                .fetch_one(&estado.pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            (itens, total.0)
        } else {
            let query = format!(
                "SELECT id, nome, preco FROM itens WHERE nome ILIKE $1 ORDER BY {} {} LIMIT $2 OFFSET $3",
                ordem_col, ordem_dir
            );
            let count_query = "SELECT COUNT(*) FROM itens WHERE nome ILIKE $1";
            
            let pattern = format!("%{}%", busca);
            let itens = sqlx::query_as::<_, Item>(&query)
                .bind(&pattern)
                .bind(por_pagina)
                .bind(offset)
                .fetch_all(&estado.pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let total: (i64,) = sqlx::query_as(count_query)
                .bind(&pattern)
                .fetch_one(&estado.pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            (itens, total.0)
        }
    } else {
        let query = format!(
            "SELECT id, nome, preco FROM itens ORDER BY {} {} LIMIT $1 OFFSET $2",
            ordem_col, ordem_dir
        );
        let count_query = "SELECT COUNT(*) FROM itens";
        
        let itens = sqlx::query_as::<_, Item>(&query)
            .bind(por_pagina)
            .bind(offset)
            .fetch_all(&estado.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let total: (i64,) = sqlx::query_as(count_query)
            .fetch_one(&estado.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        (itens, total.0)
    };

    let total_paginas = (total as f64 / por_pagina as f64).ceil() as i64;

    Ok(Json(ListarResponse {
        itens,
        total,
        pagina,
        por_pagina,
        total_paginas,
    }))
}

#[utoipa::path(
    post,
    path = "/itens",
    request_body = NovoItem,
    responses(
        (status = 201, description = "Item criado", body = Item),
        (status = 400, description = "Dados inválidos")
    )
)]
async fn criar_item(
    State(estado): State<AppState>,
    Json(payload): Json<NovoItem>,
) -> Result<(StatusCode, Json<Item>), StatusCode> {
    let novo_item = sqlx::query_as::<_, Item>(
        "INSERT INTO itens (nome, preco) VALUES ($1, $2) RETURNING id, nome, preco",
    )
    .bind(payload.nome)
    .bind(payload.preco)
    .fetch_one(&estado.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(novo_item)))
}

#[utoipa::path(
    get,
    path = "/itens/{id}",
    params(
        ("id" = u32, Path, description = "ID do item")
    ),
    responses(
        (status = 200, description = "Item encontrado", body = Item),
        (status = 404, description = "Item não encontrado")
    )
)]
async fn buscar_item(
    State(estado): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Item>, StatusCode> {
    let item = sqlx::query_as::<_, Item>(
        "SELECT id, nome, preco FROM itens WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&estado.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(item))
}

#[utoipa::path(
    put,
    path = "/itens/{id}",
    request_body = NovoItem,
    params(
        ("id" = i32, Path, description = "ID do item"),
    ),
    responses(
        (status = 200, description = "Item atualizado", body = Item),
        (status = 404, description = "Item não encontrado"),
    )
)]
async fn atualizar_item(
    State(estado): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<NovoItem>,
) -> Result<Json<Item>, StatusCode> {
    let item = sqlx::query_as::<_, Item>(
        "UPDATE itens SET nome = $1, preco = $2 WHERE id = $3 RETURNING id, nome, preco",
    )
    .bind(payload.nome)
    .bind(payload.preco)
    .bind(id)
    .fetch_optional(&estado.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(item))
}

#[utoipa::path(
    delete,
    path = "/itens/{id}",
    params(
        ("id" = i32, Path, description = "ID do item"),
    ),
    responses(
        (status = 204, description = "Item removido"),
        (status = 404, description = "Item não encontrado"),
    )
)]
async fn deletar_item(
    State(estado): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM itens WHERE id = $1",
    )
    .bind(id)
    .execute(&estado.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// OpenAPI
#[derive(OpenApi)]
#[openapi(
    paths(
        listar_itens,
        criar_item,
        buscar_item,
        atualizar_item,
        deletar_item
    ),
    components(
        schemas(Item, NovoItem, ListarParams, ListarResponse)
    ),
    info(
        title = "API Rust Simples",
        description = "API REST com documentação automática",
        version = "1.0.0"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect to database");

    let estado = AppState { pool };

    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/", get(raiz))
        .route("/itens", get(listar_itens).post(criar_item))
        .route("/itens/:id", get(buscar_item).put(atualizar_item).delete(deletar_item))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .layer(cors)
        .with_state(estado);

    println!("🚀 API rodando em http://localhost:3000");
    println!("📚 Documentação: http://localhost:3000/swagger-ui");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
