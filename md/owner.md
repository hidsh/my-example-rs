# 1. 所有権

## 所有権と型
- あらゆる整数型。u32など。
- 論理値型であるbool。trueとfalseという値がある。
- あらゆる浮動小数点型、f64など。
- 文字型であるchar。
- タプル。ただ、Copyの型だけを含む場合。例えば、(i32, i32)はCopyだが、 (i32, String)は違う。

|    |type| store | copy/move | aware ownership? |
|----|----|-------|-----------|------------------|
||bool|stack| | |
||u8.., s32, f32|stack |copy       | no |
||f32, f64|stack |copy       | no |
||char|stack|copy |no |
||char, &str|stack|copy |no |
||struct|stack + heap|move|yes|
||enum|stack + heap|move|yes|
||tuple|stack|||
||tuple|stack + heap|||


## 所有権と関数

----
- [所有権とは？ - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html)
