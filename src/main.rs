use askama::Template;
use chrono::FixedOffset;
use std::io::Write;
use std::{collections::BTreeMap, fmt::Display, path::PathBuf};





#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Activity {
    description: String,
    location: String,
    kind: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TentativeEvent {
    name: String,
    description: Option<String>,
    price: Option<Price>,
    address: Option<String>,
    url: Option<String>
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
enum ActivityEntry {
    Simple(String),
    Definite(Activity),
    Tentative {
        location: String,
        kind: String,
        events: Vec<TentativeEvent>,
    },
    // Other(serde_yaml::Value)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
enum Price {

    Known {quantity: f64,currency: String,},
    Array (f64,String),
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Price::Known { quantity, currency } => format!("{} {}", quantity, currency),
            Price::Array(quantity, currency) => format!("{} {}", quantity, currency),
        })
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Accommodation {
    check_in: chrono::DateTime<FixedOffset>,
    check_out: chrono::DateTime<FixedOffset>,
    location: String,
    short_name: String,
    url: String,
    pricing: BTreeMap<String, Vec<Price>>,
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct FixedEvent {
    location: String,
    datetime: chrono::DateTime<FixedOffset>,
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TripLeg {
    departure: FixedEvent,
    arrival: FixedEvent,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Transportation {
    short_name: String,
    url: String,
    pricing: BTreeMap<String, Vec<Price>>,
    outbound: TripLeg,
    inbound: TripLeg,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Trip {
    name: String,
    accommodations: BTreeMap<String, Accommodation>,
    travel: BTreeMap<String, Transportation>,
    activities: BTreeMap<String, Vec<ActivityEntry>>,
}

mod filters {
    use chrono::FixedOffset;

    pub fn in_timezones(dt: &chrono::DateTime<FixedOffset>, tzs: &[&str]) -> ::askama::Result<String> {
        let _dt = dt.clone();
        let local_times = tzs.into_iter().map(|ref x| _dt.with_timezone(&x.parse::<chrono_tz::Tz>().unwrap()).to_string() ).collect::<Vec<String>>().join(", ");
        Ok(format!("{}", local_times))
    }

}

#[derive(Template)]
#[template(path = "simple.html")]
struct Output {
    trip: Trip
}

fn main() -> Result<(), std::io::Error>  {
    let file = PathBuf::from("./plan.yml");
    let Ok(file) = std::fs::File::open(file) else {
        eprint!("failed to open file");
        return Ok(())
    };

    

    let parsing_result = serde_yaml::from_reader::<_, Trip>(file);
    let outfile = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open("simple.html");
    let Ok(mut outfile) = outfile else {
        eprintln!("failed to open output file");
        return Ok(())
    };

    match parsing_result {
        Ok(trip) => write!(outfile,"{}", Output{trip}.render().unwrap()),
        Err(e) => {
            eprintln!("{:?}, {}", e.location(), e);
            return Ok(())
        }
    }
    
    
}
