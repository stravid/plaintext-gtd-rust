#[derive(Clone)]
pub struct Uuid {
    value: uuid::Uuid,
}

impl Uuid {
    pub fn new() -> Self {
        Self { value: uuid::Uuid::new_v4() }
    }
}

impl std::cmp::PartialEq for Uuid {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl sqlite::Readable for Uuid {
    fn read(statement: &sqlite::Statement, i: usize) -> sqlite::Result<Uuid> {
        let result = statement.read::<String>(i).unwrap();

        sqlite::Result::Ok(Uuid { value: uuid::Uuid::parse_str(&*result).unwrap() })
    }
}

impl sqlite::Bindable for Uuid {
    fn bind(self, statement: &mut sqlite::Statement, i: usize) -> sqlite::Result<()> {
        statement.bind(i, &self.value.to_hyphenated().to_string()[..]).unwrap();
        Ok(())
    }
}