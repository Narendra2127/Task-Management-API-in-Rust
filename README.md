
# Task Management API in Rust

## Project Overview
This project is a **Task Management API** built using the **Rocket framework** in **Rust**. It provides basic CRUD operations (Create, Read, Update, Delete) for managing tasks. Each task includes an `id`, `description`, and a `completed` status. The API serves as a simple introduction to building web applications with Rust.

## Features
- **Create Task**: Add a new task with a description and completion status.
- **Get Tasks**: Retrieve a list of all tasks.
- **Update Task**: Modify a task's description or completion status.
- **Delete Task**: Remove a task by its unique `id`.

## Technologies Used
- **Rust**: The programming language used to build the API.
- **Rocket Framework**: A web framework for Rust that makes it easy to develop fast and secure web applications.
- **Serde**: A library for serializing and deserializing data in JSON format.
- **Mutex**: Ensures safe data handling across multiple requests in a concurrent environment.

## Installation

### Prerequisites:
- [Rust](https://www.rust-lang.org/) installed on your system.
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust’s package manager) is included when you install Rust.

### Steps to Run the Project:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Narendra2127/Task-Management-API.git
   ```

2. **Navigate to the project folder**:
   ```bash
   cd Task-Management-API
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run the server**:
   ```bash
   cargo run
   ```

5. **Access the API** at `http://127.0.0.1:8000`.

## API Endpoints

- **GET `/tasks`**: Fetch all tasks.
- **POST `/task`**: Create a new task.
- **PUT `/task/<id>`**: Update a task by `id`.
- **DELETE `/task/<id>`**: Delete a task by `id`.

## Example Usage
### 1. **Create a Task**:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "description": "My First Task", "completed": false}' http://127.0.0.1:8000/task
```

### 2. **Get All Tasks**:
```bash
curl http://127.0.0.1:8000/tasks
```

### 3. **Update a Task**:
```bash
curl -X PUT -H "Content-Type: application/json" -d '{"id": 1, "description": "Updated Task", "completed": true}' http://127.0.0.1:8000/task/1
```

### 4. **Delete a Task**:
```bash
curl -X DELETE http://127.0.0.1:8000/task/1
```

## Contributing
Feel free to fork the repository, make changes, and create pull requests. Contributions are always welcome!

