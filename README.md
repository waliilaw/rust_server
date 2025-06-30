# 🚀 Simple Rust Todo API

Hey there! This is a super simple todo list API built with Rust. It lets you create, view, and delete todos.

## 🌐 API URLs
- Local: `http://localhost:3000`
- Deployed: `https://rustserver-production.up.railway.app` (replace this with your actual Railway URL)

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
# Local
curl -X POST http://localhost:3000/todos \
-H "Content-Type: application/json" \
-d '{"title": "Learn Rust"}'

# Deployed
curl -X POST https://rustserver-production.up.railway.app/todos \
-H "Content-Type: application/json" \
-d '{"title": "Learn Rust"}'
```

### View All Todos
```bash
# Local
curl http://localhost:3000/todos

# Deployed
curl https://rustserver-production.up.railway.app/todos
```

### Delete a Todo
```bash
# Local
curl -X DELETE http://localhost:3000/todos/YOUR-TODO-ID

# Deployed
curl -X DELETE https://rustserver-production.up.railway.app/todos/YOUR-TODO-ID
```
(Replace YOUR-TODO-ID with the ID you got when creating the todo)

## 🔥 Using Postman?

1. **Create Todo:**
   - POST to `/todos`
   - Set `Content-Type: application/json`
   - Body: `{"title": "Your todo here"}`

2. **View Todos:**
   - GET `/todos`

3. **Delete Todo:**
   - DELETE `/todos/{id}`
   - Use the ID from the todo you want to delete

Just replace `http://localhost:3000` with your Railway URL when using the deployed version!

That's it! Have fun! 🎉
