use crate::*;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn create_course(&mut self, course_metadata: CourseMetadata) {
        let contributor_id = env::predecessor_account_id();
        let contributor_id_clone = contributor_id.clone();
        let course_id = u128::from(self.total_courses_count()) + u128::from(U128(1));
        let course = Course {
            contributor_id,
            course_id,
            metadata: course_metadata.clone()
        };

        // insert to course_metadata_by_id map
        self.course_metadata_by_id.insert(&course_id, &course_metadata);

        // insert to courses_by_contributor
        self.internal_add_course_to_contributor(&contributor_id_clone, &course);

        // insert course to courses
        self.courses.insert(&course);
    }

    pub fn register_course(&mut self, user_id: AccountId, course_id: CourseId) {

    }
}