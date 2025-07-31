#[derive(Debug)]
struct Food{
    name: String,
}
#[derive(Debug)]
struct Restaurant{
    reservations: u32,
    has_mice_infestation: bool
}
impl Restaurant{
    fn chef_special(&self)->Option<Food>{
        if self.has_mice_infestation{
            return None;
        }

        if self.reservations<12 {
            return Some(Food{name: "Uni sashimi".to_string()});
        } 
        else{
            return Some(Food{name:"Strip Steak".to_string()});
        } 
            
    }
    fn deliver_burger(&self, address:&str)-> Result<Food, String>{
        match address.is_empty() {
            true=> Err("No delivery address found".to_string()),
            false => Ok(Food{name:"Burger".to_string()})
        }
    }
}
fn main() {
    let restaurant1 = Restaurant{
        reservations:11,
        has_mice_infestation:true
    };
    println!("Invoking chef_special method {:?}", restaurant1.chef_special());
    println!("Invoking deliver_burger method {:?}",restaurant1.deliver_burger("123 Elm Street"));

    let restaurant2 = Restaurant{
        reservations:15,
        has_mice_infestation: false
    };
    println!("Invoking chef_special method {:?}", restaurant2.chef_special());
    println!("Invoking deliver_burger method {:?}",restaurant2.deliver_burger(""));
    
}
