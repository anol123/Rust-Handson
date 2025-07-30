// fn square(number:& i32)->i32{
//     number*number
// }
fn print_num(num : i32){
    println!("{num}");
}
fn print_num(num_ref : &i32){
    println!("{num_ref}");
}

fn main(){
    let mut num = 20;
    let num_ref = &num;
    print_num(num_ref);
    print_num(num);
}
