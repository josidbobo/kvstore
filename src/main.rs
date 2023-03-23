use std::collections::HashMap;
fn main() {
    let mut arguments = std::env::args().skip(1);

    // The first argument is moved then the remaining one is the next
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", key, value);

    // Only Macros can take variable number of arguments, Functions can't
    let contents = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("kv.db", contents);

    match write_result{
        Ok(_) => println!("Text successfully written in file"),
        Err(e) => println!("An error occured"),
    }

    let database_new = Database::new().expect("Database::new() crashed");
}

struct Database{
    map: HashMap<String, String>,
}

impl Database{
    fn new() -> Result<Database, std::io::Error> {

        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };

        let contents = std::fs::read_to_string("kv.db")?;

        Ok(Database{ 
            map: HashMap::new(),
         })
    }
}
 


