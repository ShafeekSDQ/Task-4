# Taskfour - Rust and PHP Time Greeting

## Description

This repository contains two simple programs:
1. A **Rust** program that prints "Hello \<your name\>, right now the time is \<current time\>".
2. A **PHP** file that outputs the same message.

Both programs display a greeting with your name and the current time.

## Files

- `main.rs`: The Rust program that prints the greeting and current time.
- `index.php`: The PHP file that prints the greeting and current time.

## Setup Instructions

### Rust Program

1. **Install Rust** if you haven't already. Follow the instructions [here](https://www.rust-lang.org/tools/install).
   
2. Create a directory:
   `mkdir taskfour`
   
3. Navigate to the directory:
   `cd taskfour`
4. Run the Rust program:
   `cargo run`
### PHP Program   
1. Ensure PHP is installed on your machine. You can check by running:
   `php --version`
2. Run the PHP file:
   `php index.php`
   This will output the greeting with the current time.
### How It Works
  - **Rust:** The program uses the chrono crate to get the current time and formats it as a string.
  - **PHP:** The program uses PHP’s built-in date() function to get the current time and display it along with a greeting.
### Rust code example
```
use chrono::Local;

fn main() {
    let name = "Your Name";
    let current_time = Local::now();
    println!("Hello {}, right now the time is {}", name, current_time.format("%Y-%m-%d %H:%M:%S"));
}
```

### PHP code example
```
<?php
date_default_timezone_set('Your/Timezone');
$name = "Your Name";
$current_time = date("Y-m-d H:i:s");
echo "Hello $name, right now the time is $current_time";
?>
```
## Upload to Github
Once you have completed both the Rust and PHP programs, upload them to a GitHub repository named **taskfour**.


