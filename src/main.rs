use serde_derive::{Deserialize, Serialize};
use serde_json;
use rand::{thread_rng, Rng};

#[derive(Deserialize, Serialize, Debug)]
struct Monitor {
    monitor_id: Option<u32>,
    name: String,
    #[serde(rename = "type")]
    mytype: Option<String>,
    script: Option<String>,
    result: Option<String>,
    code: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Monitors {
    monitors: Vec<Monitor>
}

#[derive(Deserialize, Serialize, Debug)]
struct Result1{
    value: i32,
    processed_at: i32
}

fn main() -> Result<(), std::io::Error> {
    let input_path = std::env::args().nth(1).unwrap();
    //let output_path = std::env::args().nth(2).unwrap();
    let monitor1 = {
        let monitor = std::fs::read_to_string(&input_path)?;

        // Load the Monitors structure from the string.
        serde_json::from_str::<Monitors>(&monitor).unwrap()
    };

    // Print the value of `monitor1`
    //println!("{:?}", monitor1);
    let mut rng = thread_rng();
    let mut my_instance = Result1 {
        value:0,
        processed_at: 0,
    };

    for m in monitor1.monitors {
        my_instance.value = rng.gen_range(0..100);
        my_instance.processed_at = rng.gen_range(0..100);
        let result_data = Result1 {
            value: my_instance.value,
            processed_at: my_instance.processed_at,
        };
        println!("Monitor: {:?}, Results: {:?}", m, result_data);
    }

    Ok(())

    
}
