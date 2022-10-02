export const exercises = [
  {
    id: 0,
    duration: 30, // seconds
    problem: `This code has a syntax error. Make sure it's printing the String "Number 2"`,
    starterCode: `fn main() {
    const x = 1;
    println!("Number {}", x);
}`,
    solutionCode: `fn main() {
  const x: u32 = 2; // could also be typed as i32 for example
  println!("Number {}", x);
}`,
    explanation: `Watch for the little things. Bust the compiler. It'll always beat you to it. Constants are not to be mistreated.In Rust the "const" keyword requires an explicit type!`,
  },
  {
    id: 1,
    duration: 60,
    problem:
      'The Rust compiler will show you this error: "cannot borrow `x` as mutable more than once at a time". Make the code compile only by reordering the lines, but without adding, changing or removing any of them',
    starterCode: `fn main() {
  let mut x = 100;
  let y = &mut x;
  let z = &mut x;
  *y += 100;
  *z += 1000;
  assert_eq!(x, 1200);
}
    `,
    solutionCode: `fn main() {
  let mut x = 100;
  let y = &mut x;
  *y += 100;
  let z = &mut x;
  *z += 1000;
  assert_eq!(x, 1200);
}`,
    explanation: `Carefully reason about the range in which each mutable reference is in
  scope. Does it help to update the value of referent (x) immediately after
  the mutable reference is taken?`,
  },
];
