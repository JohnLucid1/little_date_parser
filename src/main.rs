use std::{num::ParseIntError, str::FromStr};
use serde::Serialize;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono:: ParseError;

#[get("/api/{time}")]
async fn get_time(path: web::Path<String>) -> impl Responder {
    let path_time = path.into_inner();

    match path_time.contains("-") {
        true =>  {
            let res = match parse_utc_date(path_time){ 
                Ok(res) => res, 
                Err(_) => Time { utc: "NONE".to_string(), unix: "NONE".to_string()}
            };
            HttpResponse::Ok().json(res)
        },
        false => {
            let res = match parse_unix(path_time) {
                    Ok(res) => res, 
                    Err(_) => Time { utc: "ERROR".to_string(), unix: "ERROR".to_string() }
            };
            HttpResponse::Ok().json(res)
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_time)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        return self == other;
    }
}

#[derive(Debug, Serialize)]
pub struct Time {
    pub utc: String,
    pub unix: String,
}

fn parse_unix(date: String) -> Result<Time, ParseIntError> {
    use chrono::prelude::*;
    let unix_time: i64 = date.parse::<i64>()?;
    let naive =
        NaiveDateTime::from_timestamp_opt(unix_time, 0).expect("Couldn't get navei date from unix");
    let utc_datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    Ok(Time {
        utc: utc_datetime.to_string(),
        unix: unix_time.to_string(),
    })
}

fn parse_utc_date(date: String) -> Result<Time, ParseError> {
    use chrono::prelude::*;
    let naive = NaiveDate::from_str(&date)?;
    let formated_date_utc = naive.format("%a, %d %b, %C%y").to_string();
    let datetime = naive
        .and_hms_opt(0, 0, 0)
        .expect("Couldn't get date from naive");
    let unixtime = datetime.timestamp().to_string();

    Ok(Time {
        utc: formated_date_utc,
        unix: unixtime,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utc() {
        let origin = "2015-12-3".to_string();
        let res = parse_utc_date(origin).unwrap();
        assert_eq!(res.unix, "1449100800".to_string());
        assert_eq!(res.utc, "Thu, 03 Dec, 2015".to_string());
    }

    #[test]
    fn test_unix_parsing() {
        let original = "1449100800000".to_string();
        let res = parse_unix(original.clone()).unwrap();
        assert_eq!(res.utc, "+47890-03-06 00:00:00 UTC".to_string())
    }
}
