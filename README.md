# Rust Task Manager

## Overview

This is a simple **Task Manager** built as part of the 30-Day Rust Challenge. It demonstrates the use of **Ownership, Borrowing and References** to store, manage, and manipulate a collection of tasks and users. The application allows users to manage tasks and their owners interactively via a command-line interface (CLI).


## Features

- **Add Users**: Add new users to the collection by providing a unique ID and name.
- **Add Tasks**: Create new tasks with descriptions, assigned to a specific user.
- **Transfer Tasks**: Reassign tasks from one user to another.
- **Complete Tasks**: Mark tasks as completed.
- **List Tasks**: Display all tasks with their descriptions, owners, and completion status.

## Concepts Covered

- Ownership, Borrowing, and References: Fundamental Rust concepts for memory safety and data handling.
- Using **HashMaps** to store and manage data.
- Leveraging Rust's standard library for common operations.
- Structuring code with **modules** for clarity and reusability.
- Implementing interactive command-line programs.

## How to Run

1. Clone the repository:
    ```bash
    git clone https://github.com/Morg3an/rust-task-manager.git
    cd rust-task-manager
    ```
2. Build the project:
    ```bash
    cargo build
    ```
3. Run the project:
    ```bash
    cargo run
    ```

## Example usage
    ```
    Enter a user ID (or 'done' to stop):
    1
    Enter the user's name:
    Alice

    Enter a user ID (or 'done' to stop):
    done

    Enter a task description (or 'done' to stop):
    Complete the report
    Enter owner ID:
    1
    Task added successfully!

    Enter a task description (or 'done' to stop):
    done

    Current tasks:
    Task 1: Complete the report (Owner: Alice, Completed: false)

    Enter task ID to transfer:
    1
    Enter new owner ID:
    2
    Error: New owner does not exist

    Enter task ID to complete:
    1
    Task completed successfully!

    Tasks after completion:
    Task 1: Complete the report (Owner: Alice, Completed: true)
    ```

## Technologies Used
- Rust
- Cargo (Rust's package manager)

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
