/// Demonstrate that an iterable cannot be modified during iteration.

fn main() {
  let mut letters = vec![            // <1>
      "a", "b", "c"
  ];

  // letters.into_iter() is implicitly called with self parameter, causing move.
  for letter in letters {
      println!("{}", letter);
      // letters has been moved so cannot be borrowed here.
      letters.push(letter.clone());  // <2>
  }
}
