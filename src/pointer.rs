pub fn run() {
  println!("Here is pointer module!!");

  let value = 1;
  let (name, age) = get_user();

  println!("Value address is {:p}", &value);
  println!("User data : name={} age={}", name, age);
}

fn get_user() -> (&'static str, i32) {
  let user = ("foo", 250);
  return user
}

struct User<T> {
  name: T
}
