fn main() {
//     let mut a =10;
//     let mut c ="abc".to_string();
//     let mut x  = ||{
//         let b=c;
//         a=a+1;
//         println!("{}",a);

//     };
//     x();
//    // println!("{}",c);  //error will come



let mut a =vec![2,3,4,5];

let mut b = || a;
b();
// println!("{:?}",a);

// println!("{:?}",a);

}
