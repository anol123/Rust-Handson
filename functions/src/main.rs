fn main() {
    apply_to_jobs(10, "SDE-1");

    println!("{} is an even no. {}", 9, is_even(9));
    
    dbg!("{}", alphabet("sdfghjk"));
    let tup = alphabet("fghjka");
    println!("{}",tup.0);
    println!("{tup:#?}");
}

fn apply_to_jobs(number:i32, title: &str){
    println!("I am applying to {number} {title} jobs");
} 

fn is_even(number:i32) -> bool {
    //return number%2==0;
    number%2==0
}

fn alphabet(text: &str) -> (bool, bool){
    return (text.contains('a'), text.contains('z'));
}
