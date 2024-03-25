extern crate diesel;
use diesel::*;

extern crate diesel_full_text_search;
use diesel_full_text_search::*;

type DB = diesel::pg::Pg;

diesel::table! {
    foo (id) {
        id -> Int4,
        description -> Text,
    }
}

fn main() {
    let search = "bar";

    let query = foo::table.filter(to_tsvector(foo::description).matches(to_tsquery(search)));

    let sql = debug_query::<DB, _>(&query).to_string();

    println!("The sql code for `query` is:\n  {sql}\n");
}
