#![allow(dead_code)]

use authku::Client;
use std::ops::Deref;

pub struct Facility {
    library: i32,
    floor: i32,
    facility_type: i32,
    init_id: i32,
    final_id: i32,
    session_list: [&'static str; 20],
}

const DISCUSSION_ROOM: Facility = Facility {
    library: 3,
    floor: 3,
    facility_type: 21,
    init_id: 129,
    final_id: 134,
    session_list: [
        "08300930", "09301030", "10301130", "11301230", "12301330", "13301430", "14301530",
        "15301630", "16301730", "17301830", "18301930", "19302030", "20302200", "00000000",
        "00000000", "00000000", "00000000", "00000000", "00000000", "00000000",
    ],
};

const FCILITIES: [Facility; 1] = [DISCUSSION_ROOM];

fn get_facility_by_id(id: i32) -> Option<&'static Facility> {
    for facility in FCILITIES.iter() {
        if id >= facility.init_id && id <= facility.final_id {
            return Some(facility);
        }
    }
    None
}

pub struct Task {
    date: String,
    time: String,
    session: String,
    library: String,
    floor: String,
    facility_type: String,
    facility_id: String,
}

impl Task {

    pub fn new(date: &str, time: &str, facility_id: &str) -> Self {
        let facility_id: i32 = facility_id.parse().unwrap();
        let facility = get_facility_by_id(facility_id)
            .unwrap_or_else(|| panic!("Invalid facility id: {} \nNot support id", facility_id));
    
        // get the idex of time in session_list
        let mut session = 0;
        for (i, t) in facility.session_list.iter().enumerate() {
            if t == &time {
                session = i;
                break;
            }
        }
    
        Task {
            date: date.to_string(),
            time: time.to_string(),
            session: session.to_string(),
            library: facility.library.to_string(),
            floor: facility.floor.to_string(),
            facility_type: facility.facility_type.to_string(),
            facility_id: facility_id.to_string(),
        }
    }

    pub fn default(
        date: &str,
        time: &str,
        session: &str,
        library: &str,
        floor: &str,
        facility_type: &str,
        facility_id: &str,
    ) -> Self {
        Task {
            date: date.to_string(),
            time: time.to_string(),
            session: session.to_string(),
            library: library.to_string(),
            floor: floor.to_string(),
            facility_type: facility_type.to_string(),
            facility_id: facility_id.to_string(),
        }
    }


    pub fn make_book_url(&self) -> String {
        let url = format!("https://booking.lib.hku.hk/Secure/NewBooking.aspx?library={}&ftype={}&facility={}&date={}&session={}",
            self.library,
            self.facility_type,
            self.facility_id,
            self.date.replace("-", ""),
            self.session,
        );
        url
    }
}

impl From<(&str, &str, &str)> for Task {
    fn from((date, time, facility_id): (&str, &str, &str)) -> Self {
        Task::new(date, time, facility_id)
    }
}


pub struct LibClient {
    client: Client,
}

impl LibClient {
    pub fn new() -> Self {
        LibClient {
            client: Client::new(),
        }
    }

    pub async fn login(
        &self,
        uid: &str,
        password: &str,
    ) -> Result<&Self, Box<dyn std::error::Error>> {
        self.login_lib(uid, password).await?;
        Ok(self)
    }

    pub async fn book(&self, task: &Task) -> Result<&Self, Box<dyn std::error::Error>> {
        let url = task.make_book_url();
        let res = self.get(&url).send().await?;

        let body = res.text().await?;
        let viewstate = regex::Regex::new(r#"id="__VIEWSTATE" value="(.*)""#)
            .unwrap()
            .captures(&body)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let viewstategenerator = regex::Regex::new(r#"id="__VIEWSTATEGENERATOR" value="(.*)""#)
            .unwrap()
            .captures(&body)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let eventvalidation = regex::Regex::new(r#"id="__EVENTVALIDATION" value="(.*)""#)
            .unwrap()
            .captures(&body)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let main_toolkit =
            regex::Regex::new(r#"id="main_ToolkitScriptManager1_HiddenField" value="(.*)""#)
                .unwrap()
                .captures(&body)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

        let session_key = format!("ctl00$main$listSession${}", &task.session);

        let mut data: [(&str, &str); 22] = [
            ("__VIEWSTATE", viewstate),
            ("__VIEWSTATEGENERATOR", viewstategenerator),
            ("__EVENTVALIDATION", eventvalidation),
            ("main_ToolkitScriptManager1_HiddenField", main_toolkit),
            (
                "ctl00$main$ToolkitScriptManager1",
                "ctl00$main$upMain|ctl00$main$btnSubmit",
            ),
            ("__EVENTTARGET", ""),
            ("__EVENTARGUMENT", ""),
            ("__LASTFOCUS", ""),
            ("ctl00$main$ddlLibrary", &task.library),
            ("ctl00$main$ddlFloor", &task.floor),
            ("ctl00$main$ddlType", &task.facility_type),
            ("ctl00$main$ddlFacility", &task.facility_id),
            ("ctl00$main$ddlDate", &task.date),
            (&session_key, &task.time),
            ("ctl00$main$txtUserDescription", ""),
            ("ctl00$main$hBtnSubmit", ""),
            ("ctl00$main$hBtnEmail", ""),
            ("ctl00$main$txtEmail", ""),
            ("ctl00$main$hBtnResult", ""),
            ("__ASYNCPOST", "true"),
            ("ctl00$main$btnSubmit", "Submit"),
            ("", ""),
        ];

        self.post(&url).form(&data).send().await?;

        // append data

        data[20] = ("ctl00$main$btnSubmitYes", "Yes");
        data[21] = (
            "ctl00$main$ToolkitScriptManager1",
            "ctl00$main$UpdatePanel3|ctl00$main$btnSubmitYes",
        );

        let res = self.post(&url).form(&data).send().await?;

        let body = res.text().await?;

        if body.contains("Your Booking is successful") {
            Ok(self)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Booking failed",
            )))
        }
    }
}

impl Deref for LibClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     macro_rules! aw {
//         ($e:expr) => {
//             tokio_test::block_on($e)
//         };
//     }

//     #[test]
//     fn make_url() {
//         let task = Task::default("2021-09-30", "1", "1", "1", "1", "1", "1");
//         let url = task.make_book_url();
//         assert_eq!(url, "https://booking.lib.hku.hk/Secure/NewBooking.aspx?library=1&ftype=1&facility=1&date=20210930&session=1");
//     }

//     #[test]
//     fn task_mini() {
//         let task = Task::new("2021-09-30", "08300930", "129");
//         let url = task.make_book_url();
//         assert_eq!(url, "https://booking.lib.hku.hk/Secure/NewBooking.aspx?library=3&ftype=21&facility=129&date=20210930&session=0");

//         let task = Task::new("2021-09-30", "09301030", "130");
//         assert_eq!(task.facility_type, "21");
//         assert_eq!(task.facility_id, "130");
//         assert_eq!(task.session, "1");
//     }

//     #[test]
//     fn task_from() {
//         let task: Task = ("2021-09-30", "08300930", "129").into();
//         let url = task.make_book_url();
//         assert_eq!(url, "https://booking.lib.hku.hk/Secure/NewBooking.aspx?library=3&ftype=21&facility=129&date=20210930&session=0");
//     }

//     #[test]
//     fn test_readme() {
//         let client = LibClient::new();
//         aw!(
//             aw!(
//                 client.login("username", "password")
//             ).unwrap()
//             .book(&("2023-06-29","08300930","129").into())
//         ).unwrap();
//     }
// }
