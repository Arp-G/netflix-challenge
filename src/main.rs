mod data_loaders;
mod helpers;
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();

    // Serailly loading files takes 53 seconds
    // Concurrently loading files takes 17 seconds
    // Merging hashes takes ~4 seconds, 
    // so total time to load data parallelly = 21 seconds(release mode) (221 seconds in debug mode)
    let _data = data_loaders::parallel_loader(vec![
        "data/combined_data_1.txt",
        "data/combined_data_2.txt",
        "data/combined_data_3.txt",
        "data/combined_data_4.txt",
    ]);

    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
