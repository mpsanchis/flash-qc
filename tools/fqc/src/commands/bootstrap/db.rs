use postgres::{Client, NoTls};
use std::env;
use std::time::Duration;

pub fn check_db() -> Result<(), String> {
    println!("Setting up your DB...");
    let db_url = env::var("DATABASE_URL")
        .expect("Please set up env var DATABASE_URL. Have you installed 'mise'?");
    if let Ok(mut client) = Client::connect(&db_url, NoTls) {
        if let Ok(()) = client.is_valid(Duration::from_secs(1)) {
            println!("Your DB is already correctly configured for user flashqc");
            return Ok(());
        }
        return Err(String::from(
            "Your DB is configured, but connection cannot be established",
        ));
    }

    println!(
        "Could not connect to db flashqc with user flashqc. Attempting to create one with 'postgres' user"
    );
    if let Ok(mut client) = Client::connect("postgres://postgres@localhost", NoTls) {
        // check if role already there, otherwise create
        if client
            .query("SELECT * FROM pg_user WHERE usename = 'flashqc'", &[])
            .unwrap()
            .is_empty()
        {
            client
                .batch_execute("CREATE ROLE flashqc WITH LOGIN PASSWORD '1234'")
                .map_err(|e| format!("{:?}", e))?;

            println!("Correctly created user flashqc");
        }

        client
            .batch_execute("CREATE DATABASE flashqc")
            .map_err(|e| format!("{:?}", e))?;

        println!("Correctly created database flashqc");
        return Ok(());
    }
    Err(String::from(
        "Could not connect to your local postgres DB in any way. Make sure you have one up and running",
    ))
}
