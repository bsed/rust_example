fn main() {
    println!("{} days.", 31);
    println!("{0}, this {1}. {1}, this is {0}", "Alice","Bob");

    println!("{subject} {verb} {object}", 
        object="the lay dog",
        subject="the quick brown fox",
        verb="jump over");
    
    println!("{} of {:b} people kknow binary, the other half don't", 1, 2);

    println!("{number:>0width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    println!("My name is {0}, {1}, {0}", "Bound", "Lee");

}
