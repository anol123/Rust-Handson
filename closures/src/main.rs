fn main() {
    let mut a =10;
    let mut c ="abc".to_string();
    let mut x  = ||{
        let b=c;
        a=a+1;
        println!("{}",a);

    };
    x();
   // println!("{}",c);  //error will come
}
