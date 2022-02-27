use dotenv::dotenv;

mod week;
mod utils;

use utils::skip;

fn main() {
    dotenv().ok();

    let mut user = String::new();
    let mut location = String::new();

    skip();
    println!("Enter Twitter username:");
    std::io::stdin().read_line(&mut user).expect("Failed to read line");
    skip();
    println!("Enter your city for calculating time zone (leave blank for UTC):");
    std::io::stdin().read_line(&mut location).expect("Failed to read line");

    skip();
    println!("Loading tweets this can take a while depending on the amount of tweets...");
    
    let week = utils::month_tweet_times(&user, &location);
    if week.is_none() {
        return;
    }
    let week = week.unwrap();

    skip();skip();
    println!("--- Statistics for the last 4 weeks ---");
    skip();

    let amount: u32 = utils::week_amount(&week);
    
    println!("Amount of tweets: {}", amount);
    skip();

    let week_active = utils::active_time(week);
    
    println!("Most tweets at these times:");
    utils::print_week(week_active);

    skip();
}
