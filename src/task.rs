use crate::facilities::get_facility_by_id;

/// Task struct for booking
pub struct BookTask {
    date: String,
    time: String,
    session: String,
    library: String,
    floor: String,
    facility_type: String,
    facility_id: String,
}

impl BookTask {
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

        BookTask {
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
        BookTask {
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

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_time(&self) -> &str {
        &self.time
    }

    pub fn get_session(&self) -> &str {
        &self.session
    }

    pub fn get_library(&self) -> &str {
        &self.library
    }

    pub fn get_floor(&self) -> &str {
        &self.floor
    }

    pub fn get_facility_type(&self) -> &str {
        &self.facility_type
    }

    pub fn get_facility_id(&self) -> &str {
        &self.facility_id
    }
}

impl From<(&str, &str, &str)> for BookTask {
    fn from((date, time, facility_id): (&str, &str, &str)) -> Self {
        BookTask::new(date, time, facility_id)
    }
}

/// Task struct for fetching
#[derive(Debug)]
pub struct FetchTask {
    date: String,
    time: String,
    facility_name: String,
    state: String,
}

impl FetchTask {
    pub fn new(date: String, time: String, facility_name: String, state: String) -> Self {
        FetchTask {
            date,
            time,
            facility_name,
            state,
        }
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_time(&self) -> &str {
        &self.time
    }

    pub fn get_facility_name(&self) -> &str {
        &self.facility_name
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }
}
