use std::env;
use chrono::Datelike;

use super::week::Week;
use super::week::WeekAndNext;

#[tokio::main]
async fn request_time_offset(city: &str) -> Result<i8, Box<dyn std::error::Error>> {
    if city.len() <= 1 {
        return Ok(0);
    }

    let token: String = env::var("IPGEOLOCATION_API_KEY").expect("IPGEOLOCATION_API_KEY not found");

    let url: String = format!("https://api.ipgeolocation.io/timezone?apiKey={}&location={}", token, city);
    
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&res)?;

    if json["timezone_offset"].is_null() {
        return Err("Error getting time offset".into());
    }

    let offset: i8 = json["timezone_offset"].as_i64().unwrap() as i8;

    Ok(offset)
}

#[tokio::main]
async fn request_user_id(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let token: String = env::var("BEARER_TOKEN").expect("BEARER_TOKEN not found");
    let auth: String = format!("Bearer {}", token);

    let url: String = format!("https://api.twitter.com/2/users/by/username/{}", name);
    
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Authorization", auth)
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&res)?;

    if json["data"]["id"].is_null() {
        return Ok(String::new());
    }

    let id: String = json["data"]["id"].as_str().unwrap().to_string();

    Ok(id)
}

#[tokio::main]
async fn request_tweets(user_id: &str, pagination_token: String, time_offset: i8) -> Result<WeekAndNext, Box<dyn std::error::Error>> {

    let token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN not found");
    let auth: String = format!("Bearer {}", token);
    let date = chrono::Utc::now();
    let date = date.checked_sub_signed(chrono::Duration::days(28)).unwrap().format("%Y-%m-%dT%H:%M:%S.000Z").to_string();

    let url: String;
    if pagination_token.is_empty() {
        url = format!("https://api.twitter.com/2/users/{}/tweets?tweet.fields=created_at,source&start_time={}&max_results=100", user_id, date);
    } else {
        url = format!("https://api.twitter.com/2/users/{}/tweets?tweet.fields=created_at,source&start_time={}&max_results=100&pagination_token={}", user_id, date, pagination_token.as_str());
    }
    
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Authorization", auth)
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&res)?;

    let mut week_and_next = WeekAndNext::new();
    
    if json["data"].is_null() {
        return Ok(week_and_next);
    }
    let data = json["data"].as_array().unwrap();
    for object in data.iter() {
        let date = chrono::DateTime::parse_from_rfc3339(object["created_at"].as_str().unwrap()).unwrap();
        let date = date + chrono::Duration::hours(time_offset as i64);
        let weekday = date.weekday();

        let time: String = date.format("%H:%M").to_string();
        match weekday {
            chrono::Weekday::Mon => week_and_next.week.monday.push(time),
            chrono::Weekday::Tue => week_and_next.week.tuesday.push(time),
            chrono::Weekday::Wed => week_and_next.week.wednesday.push(time),
            chrono::Weekday::Thu => week_and_next.week.thursday.push(time),
            chrono::Weekday::Fri => week_and_next.week.friday.push(time),
            chrono::Weekday::Sat => week_and_next.week.saturday.push(time),
            chrono::Weekday::Sun => week_and_next.week.sunday.push(time)
        }
    }

    if !json["meta"]["next_token"].is_null() {
        week_and_next.next = json["meta"]["next_token"].as_str().unwrap().to_string();
    }

    Ok(week_and_next)
}

pub fn month_tweet_times(user: &str, city: &str) -> Option<Week> {
    let mut week_and_next = WeekAndNext::new();

    let user_id: String;
    if let Ok(res) = request_user_id(user) {
        if res.is_empty() {
            skip();
            println!("Error getting user");
            return None;
        }
        user_id = res;
    } else {
        skip();
        println!("Error getting user");
        return None;
    }

    let time_offset: i8;
    if let Ok(res) = request_time_offset(city) {
        time_offset = res;
    } else {
        skip();
        println!("Error getting your timezone. I probably don't know the city you entered.");
        return None;
    }

    loop {
        if let Ok(mut next) = request_tweets(&user_id, week_and_next.next, time_offset) {
            week_and_next.next = next.next;
            week_and_next.week.monday.append(&mut next.week.monday);
            week_and_next.week.tuesday.append(&mut next.week.tuesday);
            week_and_next.week.wednesday.append(&mut next.week.wednesday);
            week_and_next.week.thursday.append(&mut next.week.thursday);
            week_and_next.week.friday.append(&mut next.week.friday);
            week_and_next.week.saturday.append(&mut next.week.saturday);
            week_and_next.week.sunday.append(&mut next.week.sunday);
        } else {
            println!("Error getting tweets");
            break;
        }

        if week_and_next.next.is_empty() { break; }
    }

    week_and_next.week.sort();

    Some(week_and_next.week)
}

pub fn active_time(week: Week) -> Week {
    let mut week_active = Week::new();

    if week.monday.len() > 2 {
        week_active.monday = active_time_helper(week.monday);
    }
    if week.tuesday.len() > 2 {
        week_active.tuesday = active_time_helper(week.tuesday);
    }
    if week.wednesday.len() > 2 {
        week_active.wednesday = active_time_helper(week.wednesday);
    }
    if week.thursday.len() > 2 {
        week_active.thursday = active_time_helper(week.thursday);
    }
    if week.friday.len() > 2 {
        week_active.friday = active_time_helper(week.friday);
    }
    if week.saturday.len() > 2 {
        week_active.saturday = active_time_helper(week.saturday);
    }
    if week.sunday.len() > 2 {
        week_active.sunday = active_time_helper(week.sunday);
    }
    
    week_active
}

fn active_time_helper(times: Vec<String>) -> Vec<String> {
    let mut spaces: Vec<u16> = vec![];

    for i in 1..times.len() {
        let time_a: Vec<&str> = times[i-1].split(":").collect();
        let time_b: Vec<&str> = times[i].split(":").collect();

        let hour_a: u16 = time_a[0].parse().unwrap();
        let hour_b: u16 = time_b[0].parse().unwrap();

        let minute_a: u16 = time_a[1].parse().unwrap();
        let minute_b: u16 = time_b[1].parse().unwrap();

        let total_minutes_a: u16 = hour_a * 60 + minute_a;
        let total_minutes_b: u16 = hour_b * 60 + minute_b;

        let minutes_difference: u16 = total_minutes_b - total_minutes_a;

        spaces.push(minutes_difference);
    }

    let average: u16 = spaces.iter().sum::<u16>() / spaces.len() as u16;

    let mut active_times: Vec<Vec<String>> = vec![vec![]];
    for i in 0..spaces.len() {
        if spaces[i] < average {
            if active_times.last().unwrap().len() == 0 {
                active_times.last_mut().unwrap().push(times[i].clone());
                active_times.last_mut().unwrap().push(times[i+1].clone());
            } else {
                active_times.last_mut().unwrap().push(times[i+1].clone());
            }
        } else {
            active_times.push(vec![]);
        }
    }

    let mut active_time: Vec<String> = vec!["00:00".to_string(), "00:00".to_string()];
    let mut active_time_count: usize = 0;
    for i in 0..active_times.len() {
        if active_times[i].len() > active_time_count {
            active_time[0] = active_times[i][0].clone();
            active_time[1] = active_times[i].last().unwrap().clone();
            active_time_count = active_times[i].len();
        }
    }

    active_time
}

pub fn week_amount(week: &Week) -> u32 {
    let mut amount: u32 = 0;
    amount += week.monday.len() as u32;
    amount += week.tuesday.len() as u32;
    amount += week.wednesday.len() as u32;
    amount += week.thursday.len() as u32;
    amount += week.friday.len() as u32;
    amount += week.saturday.len() as u32;
    amount += week.sunday.len() as u32;
    amount
}

pub fn print_week(week: Week) {
    print_week_helper("Monday", week.monday);
    print_week_helper("Tuesday", week.tuesday);
    print_week_helper("Wednesday", week.wednesday);
    print_week_helper("Thursday", week.thursday);
    print_week_helper("Friday", week.friday);
    print_week_helper("Saturday", week.saturday);
    print_week_helper("Sunday", week.sunday);
}

fn print_week_helper(day: &str, times: Vec<String>) {
    if times.len() >= 2 {
        println!("{}: {} - {}", day, times[0], times[1]);
    } else {
        println!("{}: Less then 2 tweets", day);
    }
}

pub fn skip() {
    println!("");
}