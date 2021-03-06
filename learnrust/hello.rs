fn main() {
    //test 1
    println!("Hello World!");

    //test 2
    let x = 5 + /* 90 + */ 5;
    println!("Is 'x' 10 or 100? x = {}", x);

    //test 3
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0width$}", number=1, width=6);
}