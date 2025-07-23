
### Rust 笔记：在 `match` 语句中高效匹配 `String`

#### 场景问题

在 Rust 中，我们经常会有一个动态的、拥有所有权的 `String` 类型的变量（例如，从网络流或用户输入中读取的一行文本），但我们想用它来和一系列固定的、作为程序一部分的字符串字面量（`&'static str`）进行模式匹配。

直接将 `String` 和 `&str` 放入 `match` 语句会导致类型不匹配的编译错误。


```rust
// 错误示例 - 无法编译
let request_line: String = String::from("GET / HTTP/1.1");
match request_line {
    "GET / HTTP/1.1" => println!("OK"), // 错误：类型不匹配，期望 String，得到 &str
    _ => println!("Error"),
}
```

#### 核心解决方案

为了解决这个问题，我们需要在匹配前，先从 `String` 中获取一个 `&str` 类型的引用（字符串切片）。

**惯用写法 (Idiomatic Way): `&variable[..]`**

这是最简洁、最地道的写法。它利用切片语法来获取一个指向 `String` 全部内容的 `&str`。


```rust
let request_line: String = String::from("GET / HTTP/1.1");

// 正确的写法
let (status_line, filename) = match &request_line[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    "GET /sleep HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
};
```

**语法解析 `&request_line[..]`:**

1. `request_line[..]`: 这个切片操作获取了 `request_line` 这个 `String` 的全部内容，其结果是一个 `str` 类型的值。
    
2. `&`: 因为 `str` 是一个无固定大小的类型，不能直接持有，所以我们必须通过引用来使用它。这个 `&` 符号就创建了一个 `&str` 类型的引用。
    

最终，`match` 语句拿到的是一个 `&str`，它可以和 `&'static str` 类型的字符串字面量模式完美匹配。

#### 替代方案

**显式方法: `.as_str()`**

使用 `String` 类型的 `.as_str()` 方法可以达到完全相同的效果。这种写法更长一些，但意图可能更清晰易懂。



```rust
let request_line: String = String::from("GET / HTTP/1.1");

// 功能相同的替代写法
match request_line.as_str() {
    "GET / HTTP/1.1" => { /* ... */ },
    _ => { /* ... */ },
}
```

这两种方法在功能和性能上没有区别，选择哪一种取决于个人或团队的编码风格偏好。

---

### 总结要点

- **类型差异**: `String` 是拥有所有权的堆上字符串，而字符串字面量是 `&'static str` 类型（一个借用的切片）。
    
- **匹配前提**: `match` 语句要求被匹配的值和分支模式的类型兼容。
    
- **转换是关键**: 为了匹配 `String` 和 `&str`，必须先将 `String` 转换为一个临时的 `&str` 引用。
    
- **两种方式**:
    
    1. **`&variable[..]` (推荐，更简洁)**
        
    2. **`variable.as_str()` (更显式)**