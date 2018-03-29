extern crate open_notify_api;

fn main() {
    match open_notify_api::astros() {
        Ok(astros) => {
            for person in astros.people() {
                println!("{}", person.name());
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
}
