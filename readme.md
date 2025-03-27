# Vaulty

**Vaulty** is a lightweight CLI **command storage tool** that helps you save and retrieve commands effortlessly. Built in **Rust** with a focus on simplicity and efficiency. 🚀

![Demo](https://wqys0fziog.ufs.sh/f/itbpE5hD7Pnq4O8E6AVcYKp6ywGxsiRFIX5baE78LlnJB0g3)

<div align="center">

![Github Release](https://img.shields.io/github/v/release/risixdzn/vaulty)
![NPM Downloads](https://img.shields.io/npm/d18m/vaulty)

</div>

## Features

-   Store and retrieve frequently used commands
-   Organize commands with descriptions
-   Interactive prompts for adding, picking, and deleting commands
-   Searchable list for quick access 🔍
-   Copies selected commands to the clipboard
-   Simple and fast CLI interface
-   Built-in help for all commands

---

## Installation

### Install via npm

You can install Vaulty globally using npm:

```sh
npm i -g vaulty
```

> For now, NPM is only supported on Windows.

### Build from Source

You can build Vaulty from source, to your own platform (linux, macos...) following these steps:

```sh
# Clone the repository
git clone https://github.com/yourusername/vaulty.git
cd vaulty

# Build the project
cargo build --release

# Run Vaulty
./target/release/vaulty # On terminal
```

---

## Usage

Vaulty stores commands in a **JSON file** located in the **AppData/vaulty** folder.

### Commands

#### Save a command (`save` or `add`)

```sh
vaulty save -c "git commit -m 'your message'" -d "Commit with message"
```

If no arguments are passed, an interactive prompt will guide you through adding a command.

#### List stored commands (`list` or `ls`)

```sh
vaulty list
```

Displays a **table** with all stored commands and their descriptions.

#### Pick a command to copy to clipboard (`pick` or `copy`/`cp`)

```sh
vaulty pick
```

Shows an **interactive searchable list** to pick a command, which is then **copied to the clipboard**. 📋

#### Delete a command (`delete`, `del`, or `rm`)

```sh
vaulty delete <command_id>
```

If no ID is provided, an **interactive list** will allow you to select a command for deletion.

#### Help (`help`)

```sh
vaulty help
```

Shows **help for all commands**. You can also use:

```sh
vaulty <command> help
```

to get detailed help for a specific command. ❓

---

## Dependencies

-   **Rust** (language)
-   **Inquire** (interactive prompts)
-   **Tabled** (table rendering)

---

## Planned Features

-   Sync with Pastebin for command sharing
-   Import/export commands
-   Bulk insert or delete commands from input files or CLI

---

## Contributing

We welcome contributions! To contribute:

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -m 'Add feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Open a Pull Request

---

## License

Vaulty is licensed under the **MIT License**.

---

## Contact

For **questions** or **suggestions**, feel free to **open an issue** or reach out on my socials that you can find on [ricardo.gg](https://ricardo.gg) 😊
