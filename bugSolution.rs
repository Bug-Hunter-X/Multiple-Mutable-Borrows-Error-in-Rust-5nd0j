Several solutions exist to resolve the multiple mutable borrows issue.  Here are a few:

**1. Cloning:**

```rust
fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x; 

    y += 1;
    z += 1;
    x = y + z;
    println!("x = {}", x);
}
```
This creates copies of `x`. Changes to `y` and `z` won't directly affect `x`, so the borrow checker is satisfied.

**2. Interior Mutability (using RefCell or Cell):**

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let y = x.borrow_mut();
    let z = x.borrow_mut(); //This will still cause error if not handled properly
    *y += 1;
    *z += 1;
    println!("x = {}", *x.borrow());
}
```
RefCell provides interior mutability, but it will panic at runtime if you have multiple mutable borrows.

**3.  Refactoring to Avoid Multiple Mutable Borrows:**

The best solution is often to restructure your code to avoid the need for multiple mutable borrows. This frequently involves using other data structures or algorithmic approaches.