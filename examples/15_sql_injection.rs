use postgres::{Client, NoTls, SimpleQueryMessage};

fn main() {
    // connect to the db
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    loop {
        // get the input
        println!("Input user id to read from db...");
        let mut input = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        println!("Read user with id: {} ", input);

        // execute the query
        let sql = format!("SELECT * FROM users where id = {};", input);
        println!("Sql-Query: {} ", sql);
        let rows = match client.simple_query(&sql) {
            Ok(r) => r,
            _ => {
                println!("Could not find user with id: {}\n", input);
                continue;
            }
        };

        // get through all rows
        for row in rows.iter().filter_map(|r| match r {
            SimpleQueryMessage::Row(v) => Some(v),
            _ => None,
        }) {
            // get the user data
            let id = row.get("id").unwrap();
            let firstname = row.get("firstname").unwrap();
            let lastname = row.get("lastname").unwrap();
            let age = row.get("age").unwrap();
            let student = row.get("student").unwrap();
            println!("{id} - {firstname} {lastname} - {age} - {student}\n");
        }
    }
}