use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_add_course_to_contributor(&mut self, contributor_id: &AccountId, course: &Course) {
        let mut courses_set = self.courses_by_contributor.get(contributor_id).unwrap_or_else(|| {
            UnorderedSet::new(b'e')
        });

        courses_set.insert(course);
        self.courses_by_contributor.insert(contributor_id, &courses_set);
    }
}