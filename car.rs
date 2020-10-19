/*
MIT License

Copyright (c) 2020 Jean-Jacques François Reibel

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


struct Car{
  wheels: u32,
  doors: u32,
  cylinders: u32,
}
impl Car {
    fn addWheels (&mut self, wheelsToAdd: u32) {
      self.wheels += wheelsToAdd;
    }
    fn addDoors (&mut self, doorsToAdd: u32) {
      self.doors += doorsToAdd;
    }
    fn addCylinders (&mut self, cylindersToAdd: u32) {
      self.cylinders += cylindersToAdd;
    }
    fn deleteWheels (&mut self, wheelsToRemove: u32) {
      self.wheels -= wheelsToRemove;
    }
    fn deleteDoors (&mut self, doorsToRemove: u32) {
      self.doors -= doorsToRemove;
    }
    fn deleteCylinders (&mut self, cylindersToRemove: u32) {
      self.cylinders -= cylindersToRemove;
    }
}

fn main() {
  println!("Creating car. \n");
  let mut subaru = Car{wheels: 4, doors: 4, cylinders: 4};
  println!("{}{}{}", "Wheels check: ", subaru.wheels, "\n");
  println!("{}{}{}", "Doors check: ", subaru.doors, "\n");
  println!("{}{}{}", "Cylinders check: ", subaru.cylinders, "\n");
  println!("Adding wheel directly to car object. \n");
  subaru.wheels = 5;
  println!("{}{}{}", "Wheels check: ", subaru.wheels, "\n");
  println!("{}{}{}", "Doors check: ", subaru.doors, "\n");
  println!("{}{}{}", "Cylinders check: ", subaru.cylinders, "\n");
  println!("Removing wheel using object method. \n");
  subaru.deleteWheels(1);
  println!("{}{}{}", "Wheels check: ", subaru.wheels, "\n");
  println!("{}{}{}", "Doors check: ", subaru.doors, "\n");
  println!("{}{}{}", "Cylinders check: ", subaru.cylinders, "\n");

  println!("Hello World!");
}