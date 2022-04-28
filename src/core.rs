use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_course(&mut self, course_metadata: CourseMetadata) {
        let before_storage_usage = env::storage_usage();

        let contributor_id = env::predecessor_account_id();
        let contributor_id_clone = contributor_id.clone();
        let course_id = u128::from(self.total_courses_count()) + u128::from(U128(1));
        let course = Course {
            contributor_id,
            course_id,
            metadata: course_metadata.clone(),
        };

        // insert to course_metadata_by_id map
        self.course_metadata_by_id
            .insert(&course_id, &course_metadata);

        // insert to courses_by_contributor
        self.internal_add_course_to_contributor(&contributor_id_clone, &course);

        // insert course to courses
        self.courses.insert(&course);

        let after_storage_usage = env::storage_usage();
        refund_deposit(after_storage_usage - before_storage_usage);
    }

    #[payable]
    pub fn register_course(&mut self, account_id: AccountId, course_id: CourseId) {
        let user_id = env::predecessor_account_id();
        let courses = self.courses.to_vec().clone();
        let course = self.courses.iter().find(|x| x.course_id == course_id);

        // insert to courses_by_user
        if let Some(course) = course {
            let level = course.metadata.level;
            let price_in_near = match level {
                1 => 5,
                2 => 10,
                3 => 15,
                4 => 20,
                5 => 25,
                6 => 30,
                _ => 0
            };
            self.internal_add_course_to_user(&user_id, &course);
        }
    }

    pub fn start_learn(&mut self, account_id: AccountId, course_id: CourseId) {

    }
}
