fn main() {
    // //     let mut a =10;
    // //     let mut c ="abc".to_string();
    // //     let mut x  = ||{
    // //         let b=c;
    // //         a=a+1;
    // //         println!("{}",a);

    // //     };
    // //     x();
    // //    // println!("{}",c);  //error will come

    // let mut a =vec![2,3,4,5];

    // let mut b = || a;
    // b();
    // // println!("{:?}",a);

    // // println!("{:?}",a);

    let mut game_console = String::from("Playstation");
    let mut deleted_characters = String::new();

    let mut closure = |character| {
        let is_not_a = character != 'a';

        if is_not_a {
            true
        } else {
            deleted_characters.push(character);
            false
        }
    };

    // closure('b');
    // closure('a');

    game_console.retain(closure);
    println!("{game_console}");
    println!("{deleted_characters}");

    //the below example is to understand why mut is not added before closure when its passed to retain() function
    let a=10;
    x(a);
}

fn x(mut a: i32){
    a= a+1;
}