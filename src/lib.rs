#![allow(dead_code)]

mod facilities;
mod task;

use authku::Client;
use std::ops::Deref;
use task::Task;


/// a wrapper of authku::Client
pub struct LibClient {
    client: Client,
}

impl LibClient {
    pub fn new() -> Self {
        LibClient {
            client: Client::new(),
        }
    }

    /// login to lib.hku.hk
    pub async fn login(
        &self,
        uid: &str,
        password: &str,
    ) -> Result<&Self, Box<dyn std::error::Error>> {
        self.login_lib(uid, password).await?;
        Ok(self)
    }

    /// book a facility
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

        let session_key = format!("ctl00$main$listSession${}", task.get_session());

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
            ("ctl00$main$ddlLibrary", task.get_library()),
            ("ctl00$main$ddlFloor", task.get_floor()),
            ("ctl00$main$ddlType", task.get_facility_type()),
            ("ctl00$main$ddlFacility", task.get_facility_id()),
            ("ctl00$main$ddlDate", task.get_date()),
            (&session_key, task.get_time()),
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
