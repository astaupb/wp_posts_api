#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;

pub mod models;
pub mod schema;
pub mod cors;
pub mod pool;
pub mod get;
