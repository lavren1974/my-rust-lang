const NAME_CORSE: &str = "Ch-04";

fn main() {
    println!("NAME_CORSE: {}", NAME_CORSE);

    // // https://www.w3schools.com/charsets/ref_emoji.asp
    // let my_char: char = '✋';
    // println!("my_char: {}", my_char);

    // let my_floats: [f32; 10] = [0.5; 10];
    // println!("my_floats: {:?}", my_floats);

    // let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    // println!("my_floats_new: {:?}", my_floats_new);

    // let my_name: &str = "Nik";
    // println!("my_name: {}", my_name);
    // let mut my_full_name: String = String::from("Nik Lavren");
    // my_full_name.push('!');
    // my_full_name.push_str("!!");
    // println!("my_full_name: {:p}", &my_full_name);
    // println!("my_full_name: {}", my_full_name);

    // let name_slice: &str = &my_full_name[0..5];
    // println!("name_slice: {}", name_slice);

    // let mut my_vec: Vec<char> = Vec::new();
    // my_vec.insert(0, 'A');
    // my_vec.insert(1, 'B');
    // my_vec.push('C');
    // my_vec.push('D');
    // println!("my_vec: {:?}", my_vec);
    // dbg!(&my_vec);
    // let my_vec_del_char: char = my_vec.pop().unwrap();
    // println!("my_vec_del_char: {:}", my_vec_del_char);
    // println!("my_vec: {:?}", my_vec);

    // my_vec.iter().for_each(|c| println!("{}", c));

    // let my_vec_init: Vec<char> = vec!['a', 'b', 'c'];
    // dbg!(&my_vec_init);

    // let collect: String = my_vec_init.iter().collect();
    // dbg!(&collect);

    let num: i32 = 500;
    let new_num = |x: i32| x + num;
    let new_num_2 = new_num(300);
    dbg!(&new_num_2);

    let my_json: &str = r#"{ "message": 300 }"#;
    dbg!(&my_json);

    println!("binary: {:08b}", 22559897);
}
