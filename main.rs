use chrono::Local;

fn main() {
    let name = "Your Name";
    let current_time = Local::now();
    println!("Hello {}, right now the time is {}", name, current_time.format("%Y-%m-%d %H:%M:%S"));
}
