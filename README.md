# floatx4-rs
A library that can perform 4 float operations simultaneously

# Examples

```rust
use floatx4::Floatx4;

// let array = Floatx4::from_scalar(2.);
let array = Floatx4::from_array([1.,2.,3.,4.]);

// +,-,*,/ ...
let mut new_array = array * array ;
new_array+=array;

// print
for value in new_array{
    println!("{value:?}");
}
```
