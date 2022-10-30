﻿// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::offset::Utc;
use chrono::DateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct TeamMember {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Meeting {
    pub id: i32,
    pub name: String,
}
