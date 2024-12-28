### Rust Cheatsheet

---

### **Basics**
- **Variable Declaration**:
  ```rust
  let x = 5;           // Immutable
  let mut y = 10;      // Mutable
  ```
- **Data Types**: `i32`, `u32`, `f64`, `bool`, `char`, `String`, arrays `[i32; 3]`, tuples `(i32, bool)`
- **Functions**:
  ```rust
  fn add(x: i32, y: i32) -> i32 {
      x + y
  }
  ```

---

### **Control Flow**
- **Conditionals**:
  ```rust
  if x > 5 { println!("Big"); } else { println!("Small"); }
  ```
- **Loops**:
  ```rust
  loop { break; }       // Infinite loop
  while x < 10 { x += 1; }
  for i in 0..5 { println!("{}", i); }
  ```

---

### **Ownership & Borrowing**
- **Ownership**:
  ```rust
  let s = String::from("hello");    // Ownership moves
  let s2 = s;                      // s is invalidated
  ```
- **Borrowing**:
  ```rust
  fn print_str(s: &String) { println!("{}", s); }  // Borrow by reference
  ```
- **Mutable Borrowing**:
  ```rust
  fn change(s: &mut String) { s.push('!'); }
  ```

---

### **Common Collections**
- **Vectors**:
  ```rust
  let mut v = vec![1, 2, 3];
  v.push(4);
  ```
- **HashMap**:
  ```rust
  use std::collections::HashMap;
  let mut map = HashMap::new();
  map.insert("key", "value");
  ```

---

### **Enums & Pattern Matching**
- **Enum**:
  ```rust
  enum Option<T> { Some(T), None }
  ```
- **Match**:
  ```rust
  match x {
      1 => println!("One"),
      _ => println!("Other"),
  }
  ```

---

### **Traits & Generics**
- **Trait**:
  ```rust
  trait Greet {
      fn hello(&self);
  }
  struct Person;
  impl Greet for Person {
      fn hello(&self) { println!("Hello!"); }
  }
  ```
- **Generics**:
  ```rust
  fn largest<T: PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];
      for item in list { if item > largest { largest = item; } }
      largest
  }
  ```

---

### **Error Handling**
- **Option**:
  ```rust
  let x: Option<i32> = Some(5);
  if let Some(val) = x { println!("{}", val); }
  ```
- **Result**:
  ```rust
  let result: Result<i32, &str> = Ok(10);
  match result {
      Ok(val) => println!("Got {}", val),
      Err(e) => println!("Error: {}", e),
  }
  ```

---

### **Concurrency**
- **Threads**:
  ```rust
  use std::thread;
  let handle = thread::spawn(|| { println!("Thread!"); });
  handle.join().unwrap();
  ```
- **Channels**:
  ```rust
  use std::sync::mpsc;
  let (tx, rx) = mpsc::channel();
  tx.send(10).unwrap();
  println!("{}", rx.recv().unwrap());
  ```

---

### **Cargo Commands**
- **Create Project**: `cargo new project_name`
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: `cargo test`
- **Add Dependency**: Add to `Cargo.toml` → `[dependencies] package_name = "version"`

---

