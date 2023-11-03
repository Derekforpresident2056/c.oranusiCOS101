fn main() {
   let miles = 80.00;
   let kilometers = miles / 0.621;
   let time = 4.00;
   println!("{} miles converted to kilometers is {}",miles,kilometers);

   let speed = kilometers / time;
   println!("speed of the car is {} kilometers per hour",speed);
}
