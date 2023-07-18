
pub struct Facility {
    pub library: i32,
    pub floor: i32,
    pub facility_type: i32,
    pub init_id: i32,
    pub final_id: i32,
    pub session_list: [&'static str; 20],
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

pub const FCILITIES: [Facility; 1] = [DISCUSSION_ROOM];

pub fn get_facility_by_id(id: i32) -> Option<&'static Facility> {
    for facility in FCILITIES.iter() {
        if id >= facility.init_id && id <= facility.final_id {
            return Some(facility);
        }
    }
    None
}