### Rust Ownership Rules
- Each Value in RUST has a variable that's called it's owner.
- There can only be one owner at a time.
- When owner goes out of scope, the value will be dropped.


- Int, Bool and Char can be copied without clone method as it is stored in stack. However to copy the string we need to use `clone` method.