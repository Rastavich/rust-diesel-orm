use crate::{
    messages::{FetchMeeting, FetchTalkingPoint, FetchTeamMember, FetchTeamMemberById},
    AppState, DbActor,
};
use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

// Hello world on root path
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/api/team-members")]
pub async fn fetch_team_members(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchTeamMember).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/api/team-members/{id}")]
pub async fn fetch_team_member(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("GET /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchTeamMemberById { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
    }
}

#[get("/api/meetings")]
pub async fn fetch_meetings(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchMeeting).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No meetings found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve meetings"),
    }
}

#[get("/api/talking-points")]
pub async fn fetch_talking_points(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchTalkingPoint).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No meetings found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve meetings"),
    }
}

// #[post("/users/{id}/articles")]
// pub async fn create_user_article(state: Data<AppState>, path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
//     let id: i32 = path.into_inner();
//     // format!("POST /users/{id}/articles")

//     let db: Addr<DbActor> = state.as_ref().db.clone();

//     match db.send(CreateArticle {
//         title: body.title.to_string(),
//         content: body.content.to_string(),
//         created_by: id
//     }).await
//     {
//         Ok(Ok(info)) => HttpResponse::Ok().json(info),
//         _ => HttpResponse::InternalServerError().json("Failed to create article"),
//     }
// }
