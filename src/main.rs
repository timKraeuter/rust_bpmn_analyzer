struct BPMNModel {
    name: String
}

impl BPMNModel {
    fn wurst(&self) -> i32 {
        24
    }
}

fn main() {
    let model = BPMNModel {
        name: String::from("some user")
    };
    println!("Hello, {} {}", model.wurst(), model.name);
}
