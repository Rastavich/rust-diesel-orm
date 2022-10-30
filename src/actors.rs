use crate::db_models::{Meeting, TeamMember};
use crate::db_utils::DbActor;
// use crate::insertables::NewArticle;
use crate::messages::{FetchMeeting, FetchTeamMember, FetchTeamMemberById};
use crate::schema::meetings::dsl::*;
use crate::schema::team_members::{dsl::*, id as user_id};
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchTeamMember> for DbActor {
    type Result = QueryResult<Vec<TeamMember>>;

    fn handle(&mut self, _msg: FetchTeamMember, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch User: Unable to establish connection");

        team_members.get_results::<TeamMember>(&mut conn)
    }
}

impl Handler<FetchTeamMemberById> for DbActor {
    type Result = QueryResult<TeamMember>;

    fn handle(&mut self, msg: FetchTeamMemberById, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch User Articles: Unable to establish connection");

        team_members
            .filter(user_id.eq(msg.user_id))
            .get_result::<TeamMember>(&mut conn)
    }
}

// impl Handler<CreateArticle> for DbActor {
//   type Result = QueryResult<Article>;

//   fn handle(&mut self, msg: CreateArticle, _ctx: &mut Self::Context) -> Self::Result {
//     let mut conn = self.0.get().expect("Create User Article: Unable to establish connection");

//     let new_article = NewArticle {
//       title: msg.title,
//       content: msg.content,
//       created_by: msg.created_by,
//     };

//     diesel::insert_into(articles)
//       .values(new_article)
//       .returning((
//         article_id,
//         title,
//         content,
//         created_by,
//         created_on.nullable(),
//       ))
//       .get_result::<Article>(&mut conn)
//   }
// }
