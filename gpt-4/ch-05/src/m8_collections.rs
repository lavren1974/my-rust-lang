use std::collections::{HashMap, HashSet};

// cargo test tests_HashMap -- --nocapture


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_HashMap() {
        //dbg!("Hello tests_collections!");
        let person_1 = "Alice";
        let person_2 = "Bob";

        let mut res_hm: HashMap<&str, u32> = HashMap::new();
        res_hm.insert(person_1, 33);
        res_hm.insert(person_2, 44);

        let test_person_1 = res_hm.get(person_1);
        //dbg!(test_person_1.unwrap());

        if res_hm.contains_key("Alice2"){
            dbg!("Alice es!");
        }else{
            dbg!("Alice Not!");
        }
    }

    #[test]
    fn tests_HashSet() {
        //dbg!("Hello tests_collections!");


        let mut res_hs: HashSet<&str> = HashSet::new();
        res_hs.insert("Mikola");
        res_hs.insert("Donald");

        if res_hs.contains("Mikola2"){
            dbg!("Mikola es!");
        }else{
            dbg!("Mikola Not!");
        }

    }
}
