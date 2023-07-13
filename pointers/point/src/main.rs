struct hold{
    name:String,
}

impl Drop for hold{
    fn drop(&mut self) {
        println!("gone");
    }
}



fn main() {
{
  let x = hold{name:String::from("aakarshan")};
}
println!("hi");
}

