use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {

    use self::schema::instrument::dsl::*;

    let connection = &mut establish_connection();
    let results = instrument
        .limit(5)
        .select(Instrument::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for instr in results {
        println!("{:?}", instr.make);
        println!("-----------\n");
        println!("{:?}", instr.model);
    }
}