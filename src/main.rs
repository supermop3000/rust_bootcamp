use std::thread::current;

fn main() {
    println!("Hello, welcome to the bootcamp project!");
    fizzbuzz();
}

fn fizzbuzz() {
    println!("Running fizzbuzz function");

    let max_count = 301;
    let mut current_count = 0;
    let mut fizzbuzz_count = 0;

    // instantiate loop
    while current_count <= max_count {
        println!("count value: {}", current_count);
        if current_count % 3 == 0 && current_count != 0 {
            println!("fizz")
        }

        if current_count % 5 == 0 {
            println!("buzz")
        }

        if current_count % 3 == 0 && current_count % 5 == 0 {
            println!("fizz buzz");
            fizzbuzz_count += 1;
        }

        current_count += 1;
    }

    println!("fizz buzz occurred {} times", fizzbuzz_count);

}
