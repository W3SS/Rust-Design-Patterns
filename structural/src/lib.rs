
#[macro_use]
extern crate data_downloader_derive; 

pub mod adapter;
pub mod decorator;
pub mod bridge;
pub mod composite;
pub mod facade;
pub mod flyweight;
//pub mod proxy;

extern crate colored;

use colored::*;

pub fn adapter() {
    use adapter::*;

    println!("{}", "Adapter".green());
    let logger =  AppLogger::new();
    logger.log_info(String::from("This is an info message."));
    logger.log_debug(String::from("Debug something here."));
    logger.log_error(String::from("Show an error message."));
    logger.log_warning(String::from("About to finish."));
    logger.log_info(String::from("Bye!"));
}

pub fn decorator() {
    use decorator::desserts::*;

    println!("Decorator");

    let dessert = Crepe::new();
    let dessert = Chantilly::new(dessert);
    println!("{}", dessert.to_string());

    let dessert = Gauffre::new();
    let dessert = Chocolat::new(dessert);
    let dessert = Chantilly::new(dessert);
    println!("{}", dessert.to_string());

    let dessert = Chocolat::new(Chantilly::new(Crepe::new()));
    println!("{}", dessert.to_string());

    use decorator::input_readers::*;

    let reader = AdvancedInputReader::new("input.txt");
    let mut reader = CapitalizedInputReader::new(reader);
    let res = reader.read_lines();
    for r in res.iter() {
        println!("{}", r);
    }

    let reader = AdvancedInputReader::new("input.txt");
    let reader = CapitalizedInputReader::new(reader);
    let mut reader = LenghtInputReader::new(reader);
    let res = reader.read_lines();
    for r in res.iter() {
        println!("{}", r);
    }
    

}

pub fn bridge() {
    use bridge::key::*;

    println!("Bridge");
    let house = HouseOneDoor::new(Box::new(HouseDoorKey));
    println!("{}", house.enter());
    println!("{}", house.leave());

    let garage = CarOneDoor::new(Box::new(CarDoorKey));
    println!("{}", garage.enter());
    println!("{}", garage.leave());   

}

pub fn composite() {
    
    use composite::*;

    println!("Composite");
    let mut tree = Tree::new();
    tree.add(Box::new(Leaf::new(String::from("leaf 1"))));

    let mut subtree1 = Tree::new();
    subtree1.add(Box::new(Leaf::new(String::from("leaf 2"))));

    let mut subtree2 = Tree::new();
    subtree2.add(Box::new(Leaf::new(String::from("leaf 3"))));
    subtree2.add(Box::new(Leaf::new(String::from("leaf 4"))));
    subtree1.add(Box::new(subtree2));
    tree.add(Box::new(subtree1));

    let mut subtree3 = Tree::new();
    let mut subtree4 = Tree::new();
    subtree4.add(Box::new(Leaf::new(String::from("leaf 5"))));
    subtree4.add(Box::new(Leaf::new(String::from("leaf 6"))));

    subtree3.add(Box::new(subtree4));
    tree.add(Box::new(subtree3));

    tree.print(String::from("-"));
}

pub fn facade() {
    use facade::with_struct::*;

    println!("Facade with derive trait");
    let url = String::from("google.com");
    let reader = DataReader;
    println!("{:?}", reader.read_person(url));

}

pub fn facade_with_derive_trait() {
    use facade::with_derive_trait::*;

    println!("Facade with derive trait");
    let url = String::from("google.com");
    let reader = DataReader;
    println!("{:?}", reader.read_person(url));

}

pub fn flyweight() {
    use flyweight::*;

    println!("Flyweight");
    let mut graphic = Graphic::new();
    let mut circles = CircleFactory::new();
    graphic.add_circle(1, 1, 1.0, circles.make_circle(Color::Green));
    graphic.add_circle(1, 2, 1.0, circles.make_circle(Color::Red));
    graphic.add_circle(2, 1, 1.0, circles.make_circle(Color::Blue));
    graphic.add_circle(2, 2, 1.0, circles.make_circle(Color::Green));
    graphic.add_circle(2, 3, 1.0, circles.make_circle(Color::Yellow));
    graphic.add_circle(3, 2, 1.0, circles.make_circle(Color::Magenta));
    graphic.add_circle(3, 3, 1.0, circles.make_circle(Color::Blue));
    graphic.add_circle(4, 3, 1.0, circles.make_circle(Color::Blue));
    graphic.add_circle(3, 4, 1.0, circles.make_circle(Color::Yellow));
    graphic.add_circle(4, 4, 1.0, circles.make_circle(Color::Red));

    graphic.draw();

    println!("Total number of circle objects created: {}", circles.circles_created());

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
