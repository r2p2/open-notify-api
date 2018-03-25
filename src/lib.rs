//! Ruby library to access the web api provided by http://open-notify.org/ (CC BY 3.0 Nathan Bergey).


extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod error;

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
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
    
    pub fn number(&self) -> i32 {
        self.number
    }

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
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn latitude(&self) -> &str {
        &self.iss_position.latitude.as_str()
    }

    pub fn longitude(&self) -> &str {
        &self.iss_position.longitude.as_str()
    }
}

/// Fetch astronouts currently in space.
pub fn astros() -> Result<Astros, error::OpenNotificationError> {
    astro_from_json(&reqwest::get("http://api.open-notify.org/astros.json")?.text()?)
}

fn astro_from_json(data: &str) -> Result<Astros, error::OpenNotificationError> {
    let astros: Astros = serde_json::from_str(data)?;
    Ok(astros)
}

/// Fetch current ISS position.
pub fn iss_now() -> Result<IssNow, error::OpenNotificationError> {
    iss_now_from_json(&reqwest::get("http://api.open-notify.org/iss-now.json")?.text()?)
}

fn iss_now_from_json(data: &str) -> Result<IssNow, error::OpenNotificationError> {
    let iss_now: IssNow = serde_json::from_str(data)?;
    Ok(iss_now)
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
            assert_eq!(astros.number(), 6);
            assert_eq!(astros.people().len(), 6);
            for person in expected_people.iter() {
                assert!(astros.people().contains(&person));
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn astro_parse_faulty_data() {
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

        if let Err(_) = astro_from_json(input_data) {
            assert!(true);
        } else {
            assert!(false);
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
        }
        else {
            assert!(false);
        }
    }
}
