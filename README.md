# 🚀 Simple Rust Todo API

Hey there! This is a super simple todo list API built with Rust. It lets you create, view, and delete todos.

## 🏃‍♂️ Quick Start

1. Make sure you have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Run the server:
```bash
cargo run
```

## 📝 How to Use

### Create a Todo
```bash
curl -X POST http://localhost:3000/todos \
-H "Content-Type: application/json" \
-d '{"title": "Learn Rust"}'
```

### View All Todos
```bash
curl http://localhost:3000/todos
```

### Delete a Todo
```bash
curl -X DELETE http://localhost:3000/todos/YOUR-TODO-ID
```
(Replace YOUR-TODO-ID with the ID you got when creating the todo)

## 🔥 Using Postman?

1. **Create Todo:**
   - POST to `http://localhost:3000/todos`
   - Set `Content-Type: application/json`
   - Body: `{"title": "Your todo here"}`

2. **View Todos:**
   - GET `http://localhost:3000/todos`

3. **Delete Todo:**
   - DELETE `http://localhost:3000/todos/{id}`
   - Use the ID from the todo you want to delete

That's it! Have fun! 🎉
