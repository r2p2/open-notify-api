extern crate open_notify_api;

fn main() {
    match open_notify_api::iss_pass_times(51.0, 13.5, 440.0, 10) {
        Ok(pass_times) => {
            println!("ISS passes:");
            for pass in pass_times.passes() {
                println!("- at {} for {} seconds", pass.rise(), pass.duration());
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
