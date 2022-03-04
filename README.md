![banner](https://raw.githubusercontent.com/MrScalo/TwitterTime/main/.github/twittertime-banner2.png)

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mrscalo/TwitterTime/Rust?label=BUILD&logo=rust&style=flat-square) ![GitHub](https://img.shields.io/github/license/mrscalo/TwitterTime?label=LICENSE&logo=github&style=flat-square)

## _Interestings Twitter statistics!_

Interesting statistics of a Twitter user. Including his most active time.

1. Type some Text in the Console
2. Wait a short time
3. 🐦The statistics🐦

## Features

At the moment all statistics are from the last 4 weeks.
- Amount of tweets
- Prefered platform
- Time span per weekday with the most tweets

Many more coming soon...

## Installation

```
git clone https://github.com/MrScalo/TwitterTime.git
cd TwitterTime
```

#### Setup .env

1. Rename .env_example to .env
2. Get the Twitter BEARER_TOKEN <a href="https://developer.twitter.com/en/portal/petition/essential/basic-info">here</a>
3. Get the IPGEOLOCATION_API_KEY <a href="https://app.ipgeolocation.io">here</a>
4. Put them in the .env file

</br>

To run the slower development version...

```
cargo run
```

For the fast release version...

```
cargo build --release
./target/release/TwitterTime
```

## Usage

<img src="https://raw.githubusercontent.com/MrScalo/TwitterTime/main/.github/twittertime-input-example.png" width="600">

## License

Twitter Time is provided under the <a href="https://github.com/MrScalo/TwitterTime/blob/main/LICENSE">MIT License</a>.

**Have fun!**
