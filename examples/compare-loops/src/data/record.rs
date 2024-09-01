use super::{warehouse::Warehouse, Gender};
use rand::{thread_rng, Rng};
use lipsum::lipsum;
use uuid::{Uuid, Timestamp, NoContext};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Record {
    id:          Uuid,
    first_name:  String,
    last_name:   String,
    gender:      Gender,
    age:         u8,
    occupation:  String,
    description: String,
}

impl Record {
    pub fn new(first_name: &str, last_name: &str, gender: Gender, age: u8, occupation: &str, descr: &str) -> Self {
        Self {
            id: Uuid::new_v7(Timestamp::now(NoContext)),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            gender,
            age,
            occupation: occupation.to_string(),
            description: descr.to_string(),
        }
    }

    pub fn generate() -> Self {
        let mut rng = thread_rng();
        let gender = Warehouse::gen_gender(&mut rng);

        Self::new(
            Warehouse::gen_first_name(&mut rng, gender),
            Warehouse::gen_last_name(&mut rng),
            gender,
            rng.random(),
            Warehouse::gen_occupation(&mut rng),
            &lipsum(50),
        )
    }

    pub fn with_id(self, id: Uuid) -> Self {
        Self {
            id,
            ..self
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn id_str(&self) -> String {
        self.id.to_string()
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn occupation(&self) -> &str {
        &self.occupation
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_occupation(&mut self, occupation: &str) {
        self.occupation = occupation.to_string();
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn change_occupation(&mut self) {
        let mut rng = thread_rng();

        self.occupation = Warehouse::gen_occupation(&mut rng).to_string();
    }

    pub fn change_description(&mut self) {
        self.description = lipsum(50);
    }
}
