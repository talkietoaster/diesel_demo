use diesel::Queryable;
use diesel::Selectable;
use diesel::Insertable;
use chrono::NaiveDateTime;
use serde::Serialize;


// Post Model
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use crate::schema::posts;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

// Instrument Model
use crate::schema::instrument;

#[derive(Queryable, Selectable, Debug, Serialize, Clone)]
#[diesel(table_name = instrument)]
pub struct Instrument {
    pub id: i32,
    pub name: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub type_: Option<String>, // Maps to `type` in the database
    pub country_of_manufacture: Option<String>,
    pub serial_number: Option<String>,
    pub sku: Option<String>, // Matches the renamed "sku" field in the schema
    pub new: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub model_id: Option<i32>,
    pub line: Option<String>,
    pub picture: Option<String>,
}


#[derive(Insertable)]
#[diesel(table_name = instrument)]
pub struct NewInstrument<'a> {
    pub name: Option<&'a str>,
    pub make: Option<&'a str>,
    pub model: Option<&'a str>,
    pub type_: Option<&'a str>,
    pub country_of_manufacture: Option<&'a str>,
    pub serial_number: Option<&'a str>,
    pub sku: Option<&'a str>,
    pub new: Option<bool>,
    pub created_at: Option<&'a NaiveDateTime>, // Nullable field
    pub updated_at: Option<&'a NaiveDateTime>, // Nullable field
    pub created_by: Option<&'a str>,
    pub updated_by: Option<&'a str>,
    pub model_id: Option<i32>,
    pub line: Option<&'a str>,
    pub picture: Option<&'a str>,
}

