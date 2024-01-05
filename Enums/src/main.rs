enum CarTypes{
    Hatchback,
    Sedan,
    Suv,
    Muv,
}
fn printCars(car:CarTypes){
    match car{
        CarTypes::Hatchback => {
            println!("Small car in a segment");
        }
        CarTypes::Sedan => {
            println!("Luxury car in a segment");
        }
        CarTypes::Suv => {
            println!("Sports car in a segment");
        }
        CarTypes::Muv => {
            println!("big  car in a segment");
        }
    }
}

fn main() {
    println!("Enums");
    printCars(CarTypes::Hatchback);
    printCars(CarTypes::Sedan);
    printCars(CarTypes::Suv);
    printCars(CarTypes::Muv);
}
