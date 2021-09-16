use dotenv::dotenv;
use glob::glob;
use log::info;
use postgres::{Client, NoTls};
use seahorse::{App, Command, Context};
use std::env;
use std::fs;

mod sql;
mod template;

use template::new_template;

fn migrate_database(c: &Context) {
    info!("performing migrations");

    if c.args.is_empty() || c.args.len() != 1 {
        println!(
            "Path to migrations is required. Try something like this:\n\ndb migrate db/migrations"
        );
        std::process::exit(1);
    }

    let sql_path = &c.args[0];

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Connect to the database.
    let mut client = Client::connect(&database_url, NoTls).expect("could not connect to database");

    // Glob migrations.
    for entry in sql::get_paths(&sql_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                info!("executing migration: {:?}", path);
                let contents = fs::read_to_string(path.clone())
                    .unwrap_or_else(|_| panic!("could not read migration file: {:?}", path));
                client
                    .simple_query(contents.as_str())
                    .expect("could not execute migration");
            }
            Err(e) => println!("{:?}", e),
        }
    }

    info!("migrations complete");
}

fn seed_database(c: &Context) {
    info!("seeding database");

    if c.args.is_empty() || c.args.len() != 1 {
        println!("Path to seeds is required. Try something like this:\n\ndb seed db/seeds");
        std::process::exit(1);
    }

    let sql_path = &c.args[0];

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Connect to the database.
    let mut client = Client::connect(&database_url, NoTls).expect("could not connect to database");

    // Glob seeds.
    for entry in glob(&format!("{}/*.sql", sql_path)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                info!("executing seed: {:?}", path);
                let contents = fs::read_to_string(path.clone())
                    .unwrap_or_else(|_| panic!("could not read seed file: {:?}", path));
                client
                    .simple_query(contents.as_str())
                    .expect("could not execute seed");
            }
            Err(e) => println!("{:?}", e),
        }
    }

    info!("seeding complete");
}

fn main() {
    env_logger::init();
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("db [args]")
        .command(
            Command::new("migrate")
                .description("migrate database")
                .alias("m")
                .usage("migrate [path-to-migrations]")
                .action(migrate_database),
        )
        .command(
            Command::new("seed")
                .description("seed database")
                .alias("s")
                .usage("seed [path-to-seeds]")
                .action(seed_database),
        )
        .command(
            Command::new("new")
                .description("new migration or seed")
                .alias("n")
                .usage("new [path-to-seeds-or-migration] [name-of-migration-or-seed]")
                .action(new_template),
        );

    app.run(args);
}
