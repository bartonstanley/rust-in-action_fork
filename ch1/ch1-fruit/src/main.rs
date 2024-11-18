/// Demonstrate array index out of bounds.

fn main() {
  let fruit = vec!['🥝', '🍌', '🍇'];

  // fruit as indexes 0 - 2, so cannot access element at 4. Will result in panic.
  let buffer_overflow = fruit[4];    // <1>

  assert_eq!(buffer_overflow, '🍉')  // <2>
}
