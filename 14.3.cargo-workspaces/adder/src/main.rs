use add_one;
// error[E0432]: unresolved import `rand`
// use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
