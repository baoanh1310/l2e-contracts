use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_course(&mut self, course_metadata: CourseMetadata) -> Course {
        // let before_storage_usage = env::storage_usage();

        let contributor_id = env::predecessor_account_id();
        let contributor_id_clone = contributor_id.clone();
        let course_id = u128::from(self.total_courses_count()) + u128::from(U128(1));
        let course = Course {
            contributor_id,
            course_id,
            metadata: course_metadata.clone(),
        };

        // charge contributor
        let course_level = course_metadata.clone().level;
        let price_in_near = match course_level {
            1 => LEVEL_1,
            2 => LEVEL_2,
            3 => LEVEL_3,
            4 => LEVEL_4,
            5 => LEVEL_5,
            6 => LEVEL_6,
            _ => 0,
        };

        let price_in_yocto = u128::from(U128(price_in_near * ONE_YOCTO));
        let amount = env::attached_deposit();

        assert!(
            amount >= price_in_yocto,
            "Not enough NEAR to create this course. Price: {} NEAR",
            price_in_near
        );
        if amount > price_in_yocto {
            let refund = amount - price_in_yocto;
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        // insert to course_metadata_by_id map
        self.course_metadata_by_id
            .insert(&course_id, &course_metadata);

        // insert to courses_by_contributor
        self.internal_add_course_to_contributor(&contributor_id_clone, &course);

        // insert course to courses
        self.courses.insert(&course);

        // let after_storage_usage = env::storage_usage();
        // refund_deposit(after_storage_usage - before_storage_usage);

        course
    }

    #[payable]
    pub fn register_course(&mut self, course_id: CourseId) {
        let user_id = env::predecessor_account_id();
        let courses = self.courses.to_vec().clone();
        let course = self.courses.iter().find(|x| x.course_id == course_id);

        // insert to courses_by_user
        if let Some(course) = course {
            let level = course.metadata.level;
            let price_in_near = match level {
                1 => LEVEL_1,
                2 => LEVEL_2,
                3 => LEVEL_3,
                4 => LEVEL_4,
                5 => LEVEL_5,
                6 => LEVEL_6,
                _ => 0,
            };
            let price_in_yocto = u128::from(U128(price_in_near * ONE_YOCTO));
            let amount = env::attached_deposit();
            assert!(
                amount >= price_in_yocto,
                "Not enough NEAR to register this course. Price of this course: {} NEAR",
                price_in_near
            );
            if amount > price_in_yocto {
                let refund = amount - price_in_yocto;
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
            self.internal_add_course_to_user(&user_id, &course);
        } else {
            panic!("Course doesn't exist!");
        }
    }

    pub fn start_learn(&mut self, account_id: AccountId, course_id: CourseId) {}

    pub fn update_course_user(
        &mut self,
        account_id: AccountId,
        course_id: CourseId,
        contributor_id: AccountId,
        metadata: CourseMetadata,
    ) {
        let mut courses_set = self.courses_by_user.get(&account_id).unwrap();

        let course = courses_set
            .iter()
            .find(|x| x.course_id == course_id)
            .unwrap();
        // self.internal_remove_course_from_user(&account_id, &course);
        courses_set.remove(&course);

        let updated_course = Course {
            contributor_id,
            course_id,
            metadata,
        };

        courses_set.insert(&updated_course);
        self.courses_by_user.insert(&account_id, &courses_set);

        let lng_amount = U128(10);
        self.pay_reward_user(account_id, lng_amount);
    }
}
