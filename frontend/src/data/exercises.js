export const exercises = [
  {
    id: 1,
    duration: 30, // seconds
    problem: `Make sure it's printing the String "Number 2"`,
    starterCode: `fn main() {
    const x = 1;
    println!("Number {}", x);
}`,
    solutionCode: `fn main() {
  const x: u32 = 2; // could also be typed as i32 for example
  println!("Number {}", x);
}`,
    explanation: `In Rust the "const" keyword requires an explicit type!`,
  },
];
