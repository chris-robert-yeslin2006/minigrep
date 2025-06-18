<p align="center">
  <img src="![image](https://github.com/user-attachments/assets/fbdf541f-f778-42dd-9a55-5ee0545b208f)
" alt="Rust Logo" width="120" />
</p>

<h1 align="center">MiniGrep 🦀</h1>

<p align="center">
A minimal implementation of the classic <code>grep</code> command-line tool, written in Rust. 🔍<br>
Inspired by <a href="https://doc.rust-lang.org/book/ch12-00-an-io-project.html">Rust Book Chapter 12</a>.
</p>

---

## 📦 Features

- Search for a string in a text file (case-sensitive or insensitive)
- Clean error handling using `Result` and `Box<dyn Error>`
- Modular design with `lib.rs` for core logic and `main.rs` for entry
- Command-line parsing via `std::env`
- Fully tested with `cargo test`

---

## 🚀 Getting Started

### 🔧 Requirements

- [Rust](https://www.rust-lang.org/tools/install)

### 🔨 Installation

Clone the repository:

```bash
git clone https://github.com/your-username/minigrep.git
cd minigrep
Build the project:

bash
Copy
Edit
cargo build --release
🧪 Run the Program
🔍 Case-sensitive search
bash
Copy
Edit
cargo run "hello" sample.txt
🔍 Case-insensitive search
Enable the environment variable:

bash
Copy
Edit
CASE_INSENSITIVE=1 cargo run "hello" sample.txt
📂 Example
Given a sample.txt file:

kotlin
Copy
Edit
Rust is fun!
I love the Hello world program.
HELLO from the other side.
Running:

bash
Copy
Edit
cargo run "hello" sample.txt
Outputs:

css
Copy
Edit
I love the Hello world program.
With case-insensitive:

bash
Copy
Edit
CASE_INSENSITIVE=1 cargo run "hello" sample.txt
Outputs:

css
Copy
Edit
I love the Hello world program.
HELLO from the other side.
🧪 Running Tests
bash
Copy
Edit
cargo test
🧠 Concepts Covered
struct and impl

Result and error handling

Ownership and borrowing (&str, String)

Modules and separation of concerns

Pattern matching

Writing unit tests in Rust

📄 License
This project is licensed under the MIT License.

<p align="center" >Made with ❤️ in Rust </p> 
