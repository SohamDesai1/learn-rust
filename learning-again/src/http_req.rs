use reqwest::blocking::Client;
use serde_json::{from_str, json, to_string_pretty, Value};
fn main() {
    let client = Client::new();
    get(&client);
    post(&client);
}

fn get(client: &Client) {
    println!("Running Get fn...");
    let res = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send();

    match res {
        Ok(data) => {
            let res = from_str::<Value>(&data.text().unwrap()).unwrap();
            println!("{}", to_string_pretty(&res).unwrap());
        }

        Err(err) => {
            println!("{err}")
        }
    }
}

fn post(client: &Client) {
    let body_json = json!({"title": "foo",
    "body": "bar",
    "userId": 1});

    println!("Running Post fn...");
    let post_res = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .header("Content-Type", "application/json")
        .json(&body_json)
        .send();

    match post_res {
        Ok(data) => {
            let res = from_str::<Value>(&data.text().unwrap()).unwrap();
            println!("{}", to_string_pretty(&res).unwrap());
        }
        Err(err) => {
            println!("Request failed: {err}");
        }
    }
}
