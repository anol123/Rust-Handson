const TOUCH_DOWN: i32 = 6;
fn main() {
    let season: &str = "Spring";
    let mut points_scored: i32 = 30;

    points_scored = 35;

    let _event_time: &str = "06:00";
    let event_time = 6;

    // println!(
    //     "Season is {0}, points scored is {1}, event time is {event_time}pm, touch down as {TOUCH_DOWN}, 
    //     season is {0}, points {1}, season {}, points scored {0} ,season {1}, points scored {}",season, points_scored
    // );

    println!("{}, {season}, {}, {points_scored}, {0}, {1}, {0}", season, points_scored);//allowed
    
    println!("{0},{1},{1},{},{0},{},{1},{0},{},{TOUCH_DOWN}",season,points_scored,season); //allowed
}
