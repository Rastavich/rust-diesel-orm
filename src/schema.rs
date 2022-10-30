// @generated automatically by Diesel CLI.

diesel::table! {
    meeting_members (id) {
        id -> Int4,
        meeting_id -> Int4,
        member_id -> Int4,
    }
}

diesel::table! {
    meetings (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    talking_points (id) {
        id -> Int4,
        meeting_id -> Int4,
        member_id -> Int4,
        topic -> Varchar,
        content -> Varchar,
        time_speaking -> Int4,
    }
}

diesel::table! {
    team_members (id) {
        id -> Int4,
        name -> Varchar,
        active -> Bool,
    }
}

diesel::joinable!(meeting_members -> meetings (meeting_id));
diesel::joinable!(meeting_members -> team_members (member_id));
diesel::joinable!(talking_points -> meeting_members (meeting_id));
diesel::joinable!(talking_points -> team_members (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    meeting_members,
    meetings,
    talking_points,
    team_members,
);
