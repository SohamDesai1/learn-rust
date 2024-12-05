use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
struct F1Racer {
    name: String,
    team_name: String,
    position: usize,
    track: Track,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    laps: usize,
}
fn main() {
    serialize();
    deserialize();
}

fn serialize() {
    let racer1 = F1Racer {
        name: "Carlos Sainz".to_string(),
        team_name: "Ferrari".to_string(),
        position: 6,
        track: Track {
            name: String::from("Qatar"),
            laps: 57,
        },
    };

    let res = to_string_pretty(&racer1);
    match res {
        Ok(data) => {
            println!("{data}");
        }
        Err(err) => {
            println!("{err}")
        }
    }
}

fn deserialize() {
    let json_str = r#"{
  "name": "Carlos Sainz",
  "team_name": "Ferrari",
  "position": 6,
  "track": {
    "name": "Qatar",
    "laps": 57
  }
}"#;

    let f1_racer = from_str::<F1Racer>(&json_str);

    match f1_racer {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(err) => {
            println!("{err}")
        }
    }
}
