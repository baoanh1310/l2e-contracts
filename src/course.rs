use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Course {
    pub name: String,
    pub level: u32,
    pub luck: u32,
    pub duration: u32,
    pub is_available: bool,
    pub course_type_id: u32,
    pub boxes: Vector<Vector<Card>>,
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseType {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Card {
    pub id: u32,
    pub question: String,
    pub answer: String,
}