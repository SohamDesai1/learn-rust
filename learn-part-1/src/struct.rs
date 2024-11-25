fn main(){
    let car = Car {
        name: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2019,
        color: String::from("White"),
        price: 20000.00,
    };
    car.print_car();
}

struct Car{
    name: String,
    model: String,
    year: i32,
    color: String,
    price: f32,
}

impl Car {
    fn print_car(&self) {
        println!("{} {} {} {} {}", self.name, self.model, self.year, self.color, self.price);
    }
    
}
