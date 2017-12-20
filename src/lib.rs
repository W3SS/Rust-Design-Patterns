extern crate creational;
extern crate structural;
extern crate behavioral;
extern crate functional;

pub fn factory() {
    use creational::factory::*;
    println!("factory");
    let client_mysql: Box<DataBaseClient> = MySqlClient::new();
    let client_pgsql: Box<DataBaseClient> = PgSqlClient::new();
    client_mysql.execute_query("SELECT * FROM users");
    client_pgsql.execute_query("SELECT * FROM employees");
}

pub fn abstract_factory() {
    use creational::abstract_factory::*;
    println!("abstract factory");
    let client_mysql: DataBaseClientFactory = DataBaseClientFactory::new(MySqlFactory::new());
    let client_pgsql: DataBaseClientFactory = DataBaseClientFactory::new(PgSqlFactory::new());
    client_mysql.execute_query("SELECT * FROM users");
    client_pgsql.execute_query("SELECT * FROM employees");    
}

pub fn static_factory() {
    use creational::static_factory::*;
    println!("static factory");
    let animal: Box<Animal> = from_str("Bird");
    animal.print();
}

pub fn lazy() {
    use creational::lazy::*;
    println!("lazy");
    let mut circle = Circle::new();
    println!("The basic area for a circle with radius 2.5 is {}", circle.area(2.5, false));
    println!("The precise area for a circle with radius 2.5 is {}", circle.area(2.5, true));
    println!("The basic area for a circle with radius 6.78 is {}", circle.area(6.78, false));
    println!("The precise area for a circle with radius 6.78 is {}", circle.area(6.78, true));
 }

pub fn singleton() {
    use creational::singleton::*;
    use std::{thread, time};

    println!("singleton");
    AppRegistry::print();
    println!("Sleeping for 5 seconds.");
    thread::sleep(time::Duration::from_secs(5));
    println!("I woke up.");
    AppRegistry::add_user("1", "Laurent");
    AppRegistry::add_user("2", "Pierre");
    AppRegistry::add_user("3", "Angel");
    println!("Is user with ID=1 registred?: {}", AppRegistry::is_user_registred("1"));
    println!("Removing ID=2");
    AppRegistry::remove_user("2");
    println!("Is user with ID=2 registred?: {}", AppRegistry::is_user_registred("2"));
    println!("All users registred are: {:?}", AppRegistry::get_all_user_names());
}

pub fn builder() {
    use creational::builder::standard::*;

    println!("builder");
    let person = PersonBuilder::new()
        .set_first_name(String::from("Laurent"))
        .set_last_name(String::from("Deleris"))
        .set_age(50)
        .build();
    println!("{:?}", person);
}


pub fn builder_type_safe() {
    use creational::builder::type_safe::*;

    println!("builder type safe");
    let person = PersonBuilder::new()
        .set_first_name(String::from("Laurent"))
        .set_last_name(String::from("Deleris"))
        .set_age(50);
 
    println!("{:?}", person);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
