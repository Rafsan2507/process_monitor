use serde_derive::{Deserialize, Serialize};
use serde_json;
use rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::Write;

#[derive(Deserialize, Serialize, Debug)]
struct Monitor {
    monitor_id: Option<u64>,
    name: String,
    #[serde(rename = "type")]
    mytype: Option<String>,
    script: Option<String>,
    result: Option<Result1>,
    code: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Monitors {
    monitors: Vec<Monitor>
}

#[derive(Deserialize, Serialize, Debug)]
struct Result1{
    value: u64,
    processed_at: u64
}

fn main() -> Result<(), std::io::Error> {
    let input_path = std::env::args().nth(1).unwrap();
    
    let mut monitor1 = {
        let monitor = std::fs::read_to_string(&input_path)?;

        // Load the Monitors structure from the string.
        serde_json::from_str::<Monitors>(&monitor).unwrap()
    };


    let now = SystemTime::now();

    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let seconds_since_epoch = duration_since_epoch.as_secs();

    let mut rng = thread_rng();
    let mut my_instance = Result1 {
        value:0,
        processed_at: 0,
    };

    let mut file = File::create("D:/process_monitor/assets/output.json").expect("Failed to create file");
    let mut vector: Vec<serde_json::Value> = Vec::new();

    for m in &mut monitor1.monitors {
        my_instance.value = rng.gen_range(0..100);
        my_instance.processed_at = seconds_since_epoch;

        let result_data = Result1 {
            value: my_instance.value,
            processed_at: my_instance.processed_at,
        };
        m.result = Some(result_data);
        println!("Monitor: {:?}", m);

        vector.push(serde_json::to_value(m).unwrap());

    }
    let json_output = serde_json::to_string_pretty(&vector).unwrap();
        file.write_all(json_output.as_bytes())
        .expect("Unable to write data to file");

    Ok(())
}