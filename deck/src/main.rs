use rand::{rng, seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamond", "Spade"];
        let values = ["Ace", "1", "2", "4", "J"];

        let mut cards = vec![];

        for suit in suits{
            for value in values{
                let card = format!("{} of {}",value,suit);
                cards.push(card);
            }
        }
        let deck = Deck{cards};
        return deck;
    }
    fn shuffle(&mut self){
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}
fn square(&mut number:&mut i32) -> i32{
    number*number
}
fn main() {
    // let mut deck = Deck::new();
    //  deck.shuffle();
    // println!("Here's your deck : {:#?}", deck);

    let mut value = 23;
    let value_ref = &mut value;
    println!("Square of a no.: {}",square(&mut value));
    // println!("{}",&mut value);
    // println!("{}",&mut value);
    // println!("{}",&mut value);
    // println!("{}", value_ref);
}
