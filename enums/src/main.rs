use std::num::ParseIntError;

fn main() {
    // let present_value = Some(15);
    // let missing_value: Option<bool> = None;

    // println!("present value {}", present_value.unwrap_or(0));

    // let text ="50";
    // let text_to_num = text.parse::<String>();
    // println!("{:?}",text_to_num);
    
    let mut sauses = vec!["mayonese","ketchup","ranch"];

    if let Some(sause) = sauses.pop(){
        println!("The next sause is {sause}");
    }

}