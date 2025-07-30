#[derive(Debug)]
enum Tier{
    Gold,
    Silver,
    Platinum
}
#[derive(Debug)]
enum Subscription{
    Free,
    Basic(f64, u32),
    Premium{tier_type: Tier}
}
impl Subscription{
    fn summarise(&self){
        // if let Subscription::Free = self{
        //     println!("You have restricted access to this site");
        // }
        // else if let Subscription::Basic(price, months) = self {
        //     println!("You have limited access  to the site's premium features for {price} for {months} months.");
        // }
        // else if let Subscription::Premium{tier_type} =self{
        //     println!("You have full access to the site's premium features. Your tier is {tier_type:?}")
        // }
        match self {
            Subscription::Free => println!("You have restricted access to this site"),
            Subscription::Basic(price,months)=> println!("You have limited access  to the site's premium features for {price} for {months} months."),
            Subscription::Premium{tier_type} => println!("You have full access to the site's premium features. Your tier is {tier_type:?}")
        }
    }
}
fn main() {
    let sub1 = Subscription::Free;
    let sub2 = Subscription::Basic(300.0, 3);
    let sub3 = Subscription::Premium{tier_type: Tier::Gold};

    sub1.summarise();
    sub2.summarise();
    sub3.summarise();
}
