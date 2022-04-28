use crate::*;

#[near_bindgen]
impl Contract {

    pub fn course_info(&self, course_id: CourseId) -> Option<Course> {
        let courses = self.courses.to_vec().clone();
        let course = courses.iter().find(|&x| x.course_id == course_id);

        if let Some(course) = course {
            let metadata = self.course_metadata_by_id.get(&course_id).unwrap();
            Some(Course {
                contributor_id: AccountId::try_from(course.contributor_id.to_string().clone()).unwrap(),
                course_id,
                metadata
            })
        } else {
            None
        }
    }

    pub fn total_courses_count(&self) -> U128 {
        U128(self.course_metadata_by_id.len() as u128)
    }

    pub fn total_courses(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Course> {
        let course_keys = self.course_metadata_by_id.keys_as_vector();
        let start = u128::from(from_index.unwrap_or(U128(0)));

        course_keys.iter()
        .skip(start as usize)
        .take(limit.unwrap_or(0) as usize)
        .map(|course_id| self.course_info(course_id).unwrap())
        .collect()
    }

    pub fn courses_for_user(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Course> {
        let courses = self.courses_by_user.get(&account_id);

        let courses = if let Some(courses) = courses {
            courses
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        courses.as_vector()
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .collect()
    }

    pub fn courses_for_contributor(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Course> {
        let courses = self.courses_by_contributor.get(&account_id);

        let courses = if let Some(courses) = courses {
            courses
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        courses.as_vector()
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .collect()
    }
}
