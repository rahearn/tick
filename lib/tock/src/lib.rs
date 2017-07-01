#[ macro_use ] extern crate diesel_codegen;
pub mod schema;
pub mod models;
#[ macro_use ] extern crate diesel;
extern crate dotenv;
extern crate chrono;

use std::result;
use dotenv::dotenv;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use models::*;
use schema::timers;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var( "DATABASE_URL" )
        .expect( "DATABASE_URL expected to be set in the environment" );
    SqliteConnection::establish( &database_url )
        .expect( &format!( "Error connecting to {}", database_url ) )
}

pub fn create_timer<'a>( conn: &SqliteConnection, name: &'a str, start_entry: &'a str ) -> usize {

    let new_timer = NewTimer {
        name: name,
        start_time: Local::now().timestamp() as i32,
        start_entry: start_entry,
        running: 1,
    };

    diesel::insert( &new_timer )
        .into( timers::table )
        .execute( conn )
        .expect( "Error saving new timer" )
}

fn latest_timer<'a>( conn: &SqliteConnection, name: &'a str ) -> Result<models::Timer, diesel::result::Error> {
    use schema::timers::dsl::*;

    if name != "" {
        timers.filter( name.like( &name ) )
            .filter( running.eq( 1 ) )
            .first( &conn )
    } else {
        timers.filter( running.eq( 1 ) )
            .first( &conn )
    }
}

pub fn stop_timer<'a>( conn: &SqliteConnection, name: &'a str, end_entry: &'a str ) -> usize {
    use schema::timers::dsl::*;

    let timer = latest_timer( &conn, &name );

    match timer {
        Ok( t ) => {
            diesel::update( timers.find( &t.id ) )
                .set( (
                    running.eq( 0 ),
                    end_time.eq( Local::now().timestamp() as i32 ),
                    end_entry.eq( &end_entry )
                ) )
                .execute( &conn )
                .expect( &format!( "Unable to stop timer {}", id ) )
        },
        _ => {
            println!( "Are you sure a timer is running? Better go catch it, lol." );
        }
    }

}

pub fn list_timers<'a>( conn: &SqliteConnection ) -> QueryResult<Vec<U>> {
    use schema::timers::dsl::*;
    timers.order( id.asc() )
        .load::<Timer>( &connection )
        .expect( "Error loading timers table" );
}

pub fn check_timer<'a>( conn: &SqliteConnection ) -> {}

pub fn remove_timer<'a>( conn: &SqliteConnection ) -> {}

pub fn parse_date<'a>( ts: i32 ) -> String {
    let timestring = format!( "{:?}", ts );
    let dt: DateTime<Local> = Local.datetime_from_str( &timestring, "%s" ).unwrap();
    dt.format( "%Y-%m-%d" ).to_string()
}

pub fn parse_time<'a>( ts: i32 ) -> String {
    let timestring = format!( "{:?}", ts );
    let dt: DateTime<Local> = Local.datetime_from_str( &timestring, "%s" ).unwrap();
    if ts == 0 {
        format!( "NOW" )
    } else {
        dt.format( "%H:%M:%S" ).to_string()
    }
}

pub fn get_duration<'a>( s: i32, e: i32 ) -> String {
    let mut now: i32 = Local::now().timestamp() as i32;
    if e > s {
        now = e;
    }
    let delta = now - s;
    format!(
        "{hours:02}:{minutes:02}:{seconds:02}",
        hours=delta / 60 / 60,
        minutes=delta / 60 % 60,
        seconds=delta % 60
    )
}
