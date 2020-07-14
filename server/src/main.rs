#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};


use std::env;

#[tokio::main]
async fn print_tables() {

    

    let client = DynamoDbClient::new(Region::UsEast1);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");

                for table_name in table_name_list {
                    println!("{}", table_name);
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello my world {}!", name)
}

#[get("/hi")]
fn hi() -> String {
    format!("Hello!")
}

fn main() {

    print_tables();

    rocket::ignite()
        .mount("/", routes![hello, hi])
        .launch();
}
