# 型


## String型とスライス


```rust
# mod myutil {
#     pub mod dbg {
#         pub fn as_ptr<T: ?Sized>(r: &T) -> *const T {
#             r as *const T
#         }
#     }
#     #[macro_export]
#     macro_rules! dp {
#         ($val: expr) => {
#             println!(
#                 "=> {}: {:?} {:?}, @{:p}",
#                 stringify!($val),
#                 &$val,
#                 std::any::type_name_of_val(&$val),
#                 // Playground用に $crate から myutil に書き換えています
#                 myutil::dbg::as_ptr(&$val) 
#             );
#         };
#     }
# }
{{#include ../src/ex1-variables.rs:main_body}}
```

```rust
=> s: "foo" "alloc::string::String", @0x7fff8b67a420
=> ss: "foo" "&alloc::string::String", @0x7fff8b67a4d0
=> ssl: "foo" "&str", @0x7fff8b67a578
```
