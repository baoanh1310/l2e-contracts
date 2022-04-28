use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_add_course_to_contributor(
        &mut self,
        account_id: &AccountId,
        course: &Course,
    ) {
        let mut courses_set = self
            .courses_by_contributor
            .get(account_id)
            .unwrap_or_else(|| UnorderedSet::new(b'e'));

        courses_set.insert(course);
        self.courses_by_contributor.insert(account_id, &courses_set);
    }

    pub(crate) fn internal_add_course_to_user(&mut self, account_id: &AccountId, course: &Course) {
        let mut courses_set = self
            .courses_by_user
            .get(account_id)
            .unwrap_or_else(|| UnorderedSet::new(b'f'));

        courses_set.insert(course);
        self.courses_by_user.insert(account_id, &courses_set);
    }

}
