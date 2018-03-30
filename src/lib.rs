//! Ruby library to access the web api provided by
//! [open-notify.org](http://open-notify.org/).
//!
//! # Supported
//!
//! * Request list of people in space
//! * Request position of the ISS
//!
//! # Open
//!
//! * Request ISS pass times given a location
//!
//! # Example
//! ```
//! match open_notify_api::astros() {
//!     Ok(astros) => {
//!         println!("People in space {}", astros.people().len());
//!         for person in astros.people().iter() {
//!             println!(" - {}, {}", person.name(), person.craft());
//!         }
//!     },
//!     Err(error_msg) => {
//!         eprintln!("Ups: {:?}", error_msg);
//!     }
//! }
//! ```

extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod error;

/// People are contained in a separate type `Person`
/// to add the information in which craft they are in.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct Person {
    name: String,
    craft: String,
}

impl Person {
    pub fn new(name: &str, craft: &str) -> Person {
        Person {
            name: String::from(name),
            craft: String::from(craft),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn craft(&self) -> &str {
        self.craft.as_str()
    }
}

/// Structure containing astronouts in space.
#[derive(Deserialize, Serialize)]
pub struct Astros {
    message: String,
    number: i32,
    people: Vec<Person>,
}

impl Astros {
    /// Returns the value of the `message` field.
    ///
    /// Since all examples provided by the website
    /// show the `message` attribute filled with
    /// the value `success`, we might assume, that
    /// it would differ in error scenarios.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Returns a reference to the list of `People`
    /// in space.
    pub fn people(&self) -> &Vec<Person> {
        &self.people
    }
}

#[derive(Deserialize, Serialize)]
struct IssPosition {
    latitude: String,
    longitude: String,
}

/// Structure containing the location of the ISS.
#[derive(Deserialize, Serialize)]
pub struct IssNow {
    message: String,
    timestamp: i64,
    iss_position: IssPosition,
}

impl IssNow {
    /// Returns the value of the `message` field.
    ///
    /// Since all examples provided by the website
    /// show the `message` attribute filled with
    /// the value `success`, we might assume, that
    /// it would differ in error scenarios.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Returns the time in form of a unix timestamp
    /// when the latitude and longitude information
    /// was captured.
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Latitude of the ISS
    pub fn latitude(&self) -> &str {
        self.iss_position.latitude.as_str()
    }

    /// Longitude of the ISS
    pub fn longitude(&self) -> &str {
        self.iss_position.longitude.as_str()
    }
}

/// Fetch astronouts currently in space.
pub fn astros() -> Result<Astros, error::OpenNotificationError> {
    astro_from_json(&reqwest::get("http://api.open-notify.org/astros.json")?.text()?)
}

fn astro_from_json(data: &str) -> Result<Astros, error::OpenNotificationError> {
    let astros: Astros = serde_json::from_str(data)?;

    if astros.number as usize != astros.people.len() {
        return Err(error::OpenNotificationError::Data(String::from(
            "attribute 'number' does not match length of people field",
        )));
    }

    if astros.message() != "success" {
        return Err(error::OpenNotificationError::Data(format!(
            "attribute message indicates no success but {}",
            astros.message
        )));
    }

    Ok(astros)
}

/// Fetch current ISS position.
pub fn iss_now() -> Result<IssNow, error::OpenNotificationError> {
    iss_now_from_json(&reqwest::get("http://api.open-notify.org/iss-now.json")?.text()?)
}

fn iss_now_from_json(data: &str) -> Result<IssNow, error::OpenNotificationError> {
    let iss_now: IssNow = serde_json::from_str(data)?;

    if iss_now.message() != "success" {
        return Err(error::OpenNotificationError::Data(format!(
            "attribute message indicates no success but {}",
            iss_now.message
        )));
    }

    Ok(iss_now)
}

#[derive(Default, Deserialize, Serialize)]
struct IssPassTimesRequest {
    latitude: f32,
    longitude: f32,
    altitude: f32,
    passes: u32,
    datetime: i64,
}

#[derive(Deserialize, Serialize)]
pub struct IssPassTime {
    risetime: i64,
    duration: i64,
}

impl IssPassTime {
    pub fn rise(&self) -> i64 {
        self.risetime
    }

    pub fn duration(&self) -> i64 {
        self.duration
    }
}

/// Structure containing the location of the ISS.
#[derive(Deserialize, Serialize)]
pub struct IssPassTimes {
    message: String,
    #[serde(default)]
    reason: String,
    #[serde(default)]
    request: IssPassTimesRequest,
    #[serde(default)]
    response: Vec<IssPassTime>,
}

impl IssPassTimes {
    pub fn passes(&self) -> &[IssPassTime] {
        &self.response
    }
}

/// Request ISS pass times over a specified location
///
/// # Parameters
/// * `lat` -80 to 80 in degrees
/// * `lon` -180 to 180 in degrees
/// * `alt` 0 to 10000 in meters
/// * `n` 1 to 100; How many passes shall be included in the result.
///
/// # Example
/// ```rust
/// use open_notify_api as ona;
/// if let Ok(reply) = ona::iss_pass_times(52.5, 13.4, 10.0, 5) {
///     assert_eq!(reply.passes().len(), 5);
/// }
/// ```
pub fn iss_pass_times(
    lat: f32,
    lon: f32,
    alt: f32,
    n: u32,
) -> Result<IssPassTimes, error::OpenNotificationError> {
    iss_pass_times_from_json(&reqwest::get(
        format!(
            "http://api.open-notify.org/iss-pass.json?lat={}&lon={}&alt={}&n={}",
            lat, lon, alt, n,
        ).as_str(),
    )?.text()?)
}

fn iss_pass_times_from_json(data: &str) -> Result<IssPassTimes, error::OpenNotificationError> {
    let iss_pass_times: IssPassTimes = serde_json::from_str(data)?;

    if iss_pass_times.message != "success" {
        return Err(error::OpenNotificationError::Data(iss_pass_times.reason));
    }

    Ok(iss_pass_times)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn astro_parse_successful_data() {
        let input_data = r#"{
            "message": "success",
            "number": 6,
            "people": [
            {"name": "Anton Shkaplerov", "craft": "ISS"},
            {"name": "Scott Tingle", "craft": "ISS"},
            {"name": "Norishige Kanai", "craft": "ISS"},
            {"name": "Oleg Artemyev", "craft": "Soyuz MS-08"},
            {"name": "Andrew Feustel", "craft": "Soyuz MS-08"},
            {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]
            }"#;

        let expected_people = vec![
            Person::new("Anton Shkaplerov", "ISS"),
            Person::new("Scott Tingle", "ISS"),
            Person::new("Norishige Kanai", "ISS"),
            Person::new("Oleg Artemyev", "Soyuz MS-08"),
            Person::new("Andrew Feustel", "Soyuz MS-08"),
            Person::new("Richard Arnold", "Soyuz MS-08"),
        ];

        if let Ok(astros) = astro_from_json(input_data) {
            assert_eq!(astros.message(), "success");
            assert_eq!(astros.people().len(), 6);
            for person in expected_people.iter() {
                assert!(astros.people().contains(&person));
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn astro_parse_missing_data() {
        let input_data = r#"{
            "message": "success",
            "number": 6,
            "people": [
            {"name": "Anton Shkaplerov", "craft": "ISS"},
            {"name": "Scott Tingle", "craft": "ISS"},
            {"name": "Norishige Kanai", "craft": "ISS"},
            {"name": "Oleg Artemyev" },
            {"name": "Andrew Feustel", "craft": "Soyuz MS-08"},
            {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]
            }"#;

        match astro_from_json(input_data) {
            Err(error::OpenNotificationError::Parsing(_)) => assert!(true),
            Err(_) => assert!(false),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn astro_parse_inconsistent_data() {
        let input_data = r#"{
            "message": "success",
            "number": 5,
            "people": [
            {"name": "Anton Shkaplerov", "craft": "ISS"},
            {"name": "Scott Tingle", "craft": "ISS"},
            {"name": "Norishige Kanai", "craft": "ISS"},
            {"name": "Oleg Artemyev", "craft": "Soyuz MS-08"},
            {"name": "Andrew Feustel", "craft": "Soyuz MS-08"},
            {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]
            }"#;

        match astro_from_json(input_data) {
            Err(error::OpenNotificationError::Data(_)) => assert!(true),
            Err(_) => assert!(false),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn astro_parse_unsuccessfull_data() {
        let input_data = r#"{
            "message": "unsuccess",
            "number": 6,
            "people": [
            {"name": "Anton Shkaplerov", "craft": "ISS"},
            {"name": "Scott Tingle", "craft": "ISS"},
            {"name": "Norishige Kanai", "craft": "ISS"},
            {"name": "Oleg Artemyev", "craft": "Soyuz MS-08"},
            {"name": "Andrew Feustel", "craft": "Soyuz MS-08"},
            {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]
            }"#;

        match astro_from_json(input_data) {
            Err(error::OpenNotificationError::Data(_)) => assert!(true),
            Err(_) => assert!(false),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn iss_now_parse_successful_data() {
        let input_data = r#"{
            "iss_position": {"longitude": "73.5964", "latitude": "-34.6445"},
            "message": "success",
            "timestamp": 1521971230}"#;
        if let Ok(iss_now) = iss_now_from_json(input_data) {
            assert_eq!(iss_now.message(), "success");
            assert_eq!(iss_now.timestamp(), 1521971230);
            assert_eq!(iss_now.latitude(), "-34.6445");
            assert_eq!(iss_now.longitude(), "73.5964");
        } else {
            assert!(false);
        }
    }
}
