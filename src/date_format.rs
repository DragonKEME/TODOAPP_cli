use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono::LocalResult::Single;
use serde::{Deserialize, Serializer, Deserializer};
use crate::error::Error;

pub const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
pub const SHOW_FORMAT: &str = "%d-%m-%Y";

pub fn serialize<S>(
    date: &Option<DateTime<Utc>>,
    serializer: S,

) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{

    let s =  match date {
        Some(date) => format!("{}", date.format(FORMAT)),
        None => "1970-00-00T00:00:00.000Z".to_string()
    };
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };

    match NaiveDateTime::parse_from_str(&s, FORMAT) {
        Ok(date) => {Ok(Some(Utc.from_utc_datetime(&date)))}
        Err(_) => {Ok(None)}
    }

}

pub fn showed_date_time(date: Option<DateTime<Utc>>) -> String{
    match date {
        Some(date) => date.format(SHOW_FORMAT).to_string(),
        None => String::new()
    }
}

pub fn input_date(year: String, month: String, day: String) -> Result<Option<DateTime<Utc>>, Error> {

    // If everything empty: it's valid, just none
    if year.is_empty() && month.is_empty() && day.is_empty(){
        return Ok(None)
    }

    //If something exist, year must exist
    let year: i32 = if !year.is_empty() {
            match year.parse::<i32>() {
                Ok(y) => y,
                Err(_) => return Err(Error::InputDateError("Invalid year".to_string())),
            }
        }else {
            return Err(Error::InputDateError("Year is empty but not other date fields".to_string()))
        };

    // If year exist month can be unset but only if day is also unset (date = 01/01/YYYY)
    let month: u32 = if month.is_empty() {
            if day.is_empty() {
                1
            }else {
                return Err(Error::InputDateError("Month is empty but not days".to_string()))
            }
        }else {
            match month.parse::<u32>() {
                Ok(m) => m,
                Err(_) => return Err(Error::InputDateError("Invalid month".to_string())),
            }
        };

    //if year and month exist day can be unset (date = 01/MM/YYYY)
    let day: u32 = if day.is_empty() {
            1
        }else {
            match day.parse::<u32>() {
                Ok(d) => d,
                Err(_) => return Err(Error::InputDateError("Invalid day".to_string())),
            }
        };

    //Finally create and check if the date is valid
    match Utc.with_ymd_and_hms(year,month,day,0,0,0) {
        Single(date) => Ok(Some(date)),
        _ => Err(Error::InputDateError("Invalid input".to_string()))
    }
}