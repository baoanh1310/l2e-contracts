use crate::*;

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Course {
    pub contributor_id: AccountId,
    pub course_id: CourseId,
    pub metadata: CourseMetadata,
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseMetadata {
    pub name: String,
    pub level: u32,
    pub luck: u32,
    pub start_time: u64,
    pub end_time: u64,
    pub current_date: u32,
    pub course_type_id: u32,
    pub boxes: Vec<CustomBox>,
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseType {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Card {
    pub id: u32,
    pub question: String,
    pub answer: String,
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CustomBox {
    pub cards: Vec<Card>,
}

impl Course {
    pub fn is_ended(&self) -> bool {
        self.metadata.end_time < env::block_timestamp()
    }
}
