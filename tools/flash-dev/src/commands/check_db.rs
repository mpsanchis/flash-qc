use std::time::Duration;

use postgres::{Client, NoTls};

#[derive(Debug, clap::Args)]
pub struct CheckDB {

}

impl CheckDB {
  pub fn run(self) -> Result<(), String> {
    println!("Running check-db command...");
    if let Ok(mut client) = Client::connect("postgres://flashqc:1234@localhost/flashqc", NoTls) {
      if let Ok(()) = client.is_valid(Duration::from_secs(1)) {
        println!("Your DB is already correctly configured for user flashqc");
        return Ok(());
      }
      return Err(String::from("Your DB is configured, but connection cannot be established"));
    }

    println!("Could not connect to db flashqc with user flashqc. Attempting to create one with 'postgres' user");
    if let Ok(mut client) = Client::connect("postgres://postgres@localhost", NoTls) {
      // check if role already there, otherwise create
      if client.query("SELECT * FROM pg_user WHERE usename = 'flashqc'", &[]).unwrap().len() == 0 {
        client
          .batch_execute("CREATE ROLE flashqc WITH LOGIN PASSWORD '1234'")
          .map_err(|e| format!("{:?}", e))?;

        println!("Correctly created user flashqc");
      }

      client
        .batch_execute("CREATE DATABASE flashqc")
        .map_err(|e| format!("{:?}", e))?;

      println!("Correctly created database flashqc");
    }
    Ok(())
  }
}