# **Rust Shell (MyShell) 🚀**
A lightweight, interactive shell built in Rust, featuring custom commands, autocompletion, and history support.

<br />

 ##  Features ✨
 
✅ Basic Shell Commands: Supports built-in commands like echo, exit, cd, pwd, cat, and more.

✅ Autocompletion: Press <TAB> to autocomplete commands (echo, exit, etc.).

✅ Command Execution: Run system commands (like ls, pwd, mkdir, etc.).

✅ Custom Error Handling: Provides meaningful error messages.

✅ History Support: Use the arrow keys to navigate command history.

✅ Path Management: Includes /tmp/quz/ in PATH dynamically.

<br />

# 🛠 Installation

## 1️⃣ Clone the Repository


```bash
git clone https://github.com/your-username/myshell.git
cd myshell
```
<br />

## 2️⃣ Install Rust (If Not Installed)

Ensure you have Rust installed. If not, install it using:
<br />

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
<br />

## 3️⃣ Build the Project



```bash
cargo build --release
```
<br />

## 4️⃣ Run the Shell



```bash
./run.sh
```

<br />


## 💡 Usage
### Once inside the shell, try out the commands:
```bash
$ echo Hello, World!
Hello, World!

$ pwd
/home/user/myshell

$ cd /tmp
$ pwd
/tmp

$ cat myfile.txt
(contents of myfile.txt)

$ ls
(list of files)
```
<br />

## 📂 Project Structure


```bash
myshell/
│── src/
│   ├── main.rs          # Main entry point
│   ├── commands.rs      # Handles built-in commands
│   ├── utils.rs         # Utility functions
│   ├── completion.rs    # Autocomplete logic
│── Cargo.toml           # Rust dependencies & config
│── README.md            # Documentation
```

<br />

## 🧩 Dependencies

### This project uses:

`clap` - Command-line argument parsing

`rustyline` - Interactive shell support (autocomplete, history)

`dirs` - Manage directories dynamically

<br />

### Install dependencies with:


```bash
cargo check
```

<br />

## 🤝 Contributing

### 🚀 Contributions are welcome!

1.Fork the repo

2.Create a new branch

3.Make changes & commit

4.Submit a Pull Request

<br />
<br />
