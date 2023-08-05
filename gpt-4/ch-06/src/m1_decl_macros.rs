#[cfg(test)]
mod test {
    //use super::*;

    macro_rules! my_macros_1 {
        ($x: expr) => {
            format!("my_macros_1: {}", $x)
        };
    }

    macro_rules! my_vec {
        ( $($x: expr),+ ) => {
          {
            let mut temp_vec = Vec::new();
            $(
              temp_vec.push($x);
            )+
            temp_vec
          }
        }
      }

    #[test]
    fn tests_m1_decl_macros() {
        // dbg!("Test tests_m1_decl_macros!");
        // let res = my_macros_1!(55 + 7);
        // dbg!(res);

        let res = my_vec!(4, 5, 6, 7, 8);
        dbg!(res);
    }
}

// MACRO CAPTURES

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// REPITITION SPECIFIER

// * - match zero or more repititions
// + - match one or more repititions
// ? - Match zero or one repetition
