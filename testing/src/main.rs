

fn main() {
    println!("hello");

    other_fun(56, 't');
}

fn other_fun(thing: i32, other_thing: char) {
    println!("world {} {}", thing, other_thing);
}