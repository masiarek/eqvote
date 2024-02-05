#[derive(Debug)]
pub enum ElectionModeType {
    PlayMode, // Low Security
    TestMode, // Medium Security
    RealMode, // High Security
}
/// ElectionStatus is an enum with 4 variants
/// Draft, Released, Started, Ended
/// - Released means the election is waiting for start date
/// - Started means the election is in progress
#[derive(Debug)]
pub enum ElectionStatus {
    Draft,
    Released,
    Started,
    Ended,
}
#[derive(Debug)]
pub struct Election {
    pub election_id: u32,
    pub title: String,
    pub election_mode: ElectionModeType,
    pub status: ElectionStatus,
}
