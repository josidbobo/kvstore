use std::{collections::HashMap};
fn main() {

    let mut arguments = std::env::args().skip(1);
    println!("{:?}", arguments);

    // The first argument is moved then the remaining one is the next
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", key, value);

    // Only Macros can take variable number of arguments, Functions can't
    let contents = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("kv.db", contents);
    // You can use .unwrap() for the line above to replace the match statement

    match write_result{
        Ok(_) => println!("Text successfully written in file"),
        Err(e) => println!("{}", e),
    }
    
    let database_new = Database::new().expect("Database::new() crashed");
    //database_new.insert(key, value);
    Database::insert(database_new, key, value);
}
     
#[derive(Debug)]
struct Database{
    map: HashMap<String, String>,
}

impl Database{
    fn new() -> Result<Database, std::io::Error> {
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(e) => {
        //         return Err(e);
        //     }};
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines(){
            let (key, value) = line.split_once('\t').expect("Corrupt Database");
            map.insert(String::from(key), value.to_owned());  // to_string() also works
        }

        Ok(Database{ 
            map,
         }) 
    } 

    fn insert(mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // fn get_all(mut self) -> Self{
    //     for (k, v) in self.map.iter(){
    //         return (k, v);
    //     }
    // }

}
 

mod documentation{
/** 
 * String -> This is a Struct that contains a. The pointer  b. The length of string literal c. The capacity on the heap
 * What it does is it helps to grow the string 
 * while
 * &str -> This only contains the pointer and length, it is not growable just a reference to data.
 */
fn _doc(){}
}

