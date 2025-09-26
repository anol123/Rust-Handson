use std::collections::HashMap;

trait Accomodation {
    fn get_description(&self) -> String{
        String::from("A wonderful place to stay")
    }
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarise(&self) -> String{
        format!("{}: {}",self.name, self.get_description())
    }
}
impl Accomodation for Hotel {
    // fn get_description(&self) -> String {
    //     format!("{} is the pinnacle of luxury", self.name)
    // }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB{
    host: String,
    guests:Vec<(String, u32)>
}
impl AirBnB{
    fn new(host: &str) -> Self{
        Self{
            host: host.to_string(),
            guests:vec![],
        }
    }
}
impl Accomodation for AirBnB{
    fn get_description(&self) -> String {
       format!("Please enjoy {}'s appartment", self.host) 
    }
    fn book(&mut self, name: &str, nights:u32){
        self.guests.push((name.to_string(), nights));
    }
}

fn book_for_one_night(entity: &mut impl Accomodation, guest: &str){
    entity.book(guest,1);
}

// fn mix_and_match(first: &mut impl Accomodation, second: &mut impl Accomodation, guest: &str){
//     first.book(guest, 1);
//     second.book(guest, 1);
// }
fn mix_and_match<T: Accomodation, U:Accomodation>(first: &mut T, second: &mut U, guest: &str){
    first.book(guest, 1);
    second.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("Roy's");
    println!("{}",hotel.get_description());
    hotel.book("Anol", 3);
    hotel.book("Alex", 2);

    println!("{:#?}", hotel);
    println!("{}",hotel.summarise());


    // let mut a =10;
    // println!("{}", a);
    // println!("{}", a);

    let mut airbnb = AirBnB::new("Alexander");
    println!("{}", airbnb.get_description());

    println!("{:#?}",airbnb);

    airbnb.book("Anol", 2);

    println!("{:#?}", airbnb);


    book_for_one_night(&mut hotel, "Alex");

    println!("{:#?}",hotel);

    book_for_one_night(&mut airbnb, "Peter");

    println!("{:#?}",airbnb);

    mix_and_match(&mut hotel, &mut airbnb, "Alice");
    println!("{:#?}",hotel);
    println!("{:#?}",airbnb);

}
