use dotenv::dotenv;

mod week;
mod utils;

use utils::skip;

fn main() {
    dotenv().ok();

    let mut user = String::new();
    let mut location = String::new();
    let mut week_amount = String::new();

    skip();
    println!("Enter Twitter username:");
    std::io::stdin().read_line(&mut user).expect("Failed to read line");
    skip();
    println!("Enter your city for calculating time zone (leave blank for UTC):");
    std::io::stdin().read_line(&mut location).expect("Failed to read line");
    skip();
    println!("Amount of weeks to go back (1-52):");
    std::io::stdin().read_line(&mut week_amount).expect("Failed to read line");

    let mut week_amount_num: i8 = utils::week_amount_to_int(week_amount);

    skip();
    println!("Loading tweets this can take a while depending on the amount of tweets...");
    
    let week_meta = utils::month_tweet_times(&user, &location, week_amount_num);
    if week_meta.is_none() {
        return;
    }
    let week_meta = week_meta.unwrap();
    let week = week_meta.week;

    let amount: u32 = utils::week_amount(&week);

    if amount == 3250 {
        skip();
        println!("Unfortunately the Twitter API allows no more than 3250 tweets.");
        println!("So here are the statistics for the maximum amount of tweets you can get.");
        week_amount_num = utils::week_diff(week_meta.last_date.as_str());
    }
    
    let platform = week_meta.platforms.iter().max_by_key(|&(_, v)| v).unwrap().0;

    skip();skip();
    println!("--- Statistics for the last {} week{} ---", week_amount_num, if week_amount_num == 1 { "" } else { "s" });
    skip();
    
    println!("Amount of tweets: {}", amount);
    skip();

    println!("Prefered platform: {}", platform);
    skip();

    let week_active = utils::active_time(week);
    
    println!("Most tweets at these times:");
    utils::print_week(week_active);

    skip();
}
