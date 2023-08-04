#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    email: String,
    age: u8,
    status: bool,
}

impl User {
    fn change_name(&mut self, name:String) {
        self.name = name;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        //dbg!("Hello tests_structs!");

        let mut user_1 = User {
            name: String::from("Nik Lavren"),
            email: String::from("lavren@example.com"),
            age: 49,
            status: true,
        };

        let mut user_2 = User {
            name: String::from("Nik Lavren2"),
            email: String::from("lavren2@example.com"),
            age: 49,
            status: true,
        };
        //user.name = "Nik Lavrenov".to_string();
        //user.change_name("AAAAAAA".to_string());
        dbg!(user_1);
    }
}
