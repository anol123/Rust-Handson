#[derive(Debug)]
struct Flight{
    origin: String,
    destination: String,
    price : f64,
    passengers: u32,
}
impl Flight{
    fn new() -> Self{
        Flight{
            origin: String::from("Hyd"),
            destination:String::from("Silchar"),
            price : 12000.00,
            passengers: 100,
        }
    }
    fn change_destination(&mut self, destination: String){
        self.destination = destination;
    }
    fn increase_price(&mut self){
        self.price *= 1.20;
    }
    fn itenary(&self){
        println!("Origin {} -> Destination {}", self.origin, self.destination);
    }
}

fn main() {
    let mut flight1 = Flight:: new();
    println!("{flight1:#?}");

    flight1.change_destination("Ghy".to_string());
    flight1.increase_price();
    flight1.itenary();

    println!("Updated flight1 :{flight1:#?}");

    let flight2 = Flight{
        origin: "Bnglr".to_string(),
        destination: "Silchar".to_string(),
        ..flight1
    };
    println!("New flight :{flight2:#?}");
    println!("{}",flight1.origin);
}
