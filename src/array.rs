pub fn run() {
  let mut a1 = [1, 2, 3, 4, 5];
  let mut a2 = vec![9, 10];

  a1.reverse();

  a2.insert(a2.len(), 4);
  a2.remove(0);

  println!("{:?}", a1.len());
  println!("{:?}", a2);
}
