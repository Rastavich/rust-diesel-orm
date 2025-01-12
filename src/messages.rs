use crate::db_models::{Meeting, TalkingPoint, TeamMember};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<TeamMember>>")]
pub struct FetchTeamMember;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Meeting>>")]
pub struct FetchMeeting;

#[derive(Message)]
#[rtype(result = "QueryResult<TeamMember>")]
pub struct FetchTeamMemberById {
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<TalkingPoint>>")]
pub struct FetchTalkingPoint;

// #[derive(Message)]
// #[rtype(result = "QueryResult<Article>")]
// pub struct CreateArticle {
//   pub title: String,
//   pub content: String,
//   pub created_by: i32,
// }
