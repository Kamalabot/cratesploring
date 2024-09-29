To demonstrate how to build a **CRUD (Create, Read, Update, Delete)** operation for **employee details** using **SurrealDB** with **Axum** in Rust, I'll guide you through the setup and example step-by-step.

### **Steps Overview**

1. **Setup the environment** with the required dependencies.
2. **Connect to SurrealDB**.
3. Implement **CRUD operations** using Axum.
4. Define **employee details** schema.
5. Build the **Axum routes** to handle employee CRUD.
6. Test with a basic **HTTP client** like `curl` or `Postman`.

---

### **Step 1: Setup Your Environment**

#### **1.1 Cargo.toml Dependencies**

Make sure your `Cargo.toml` file includes the necessary dependencies for **Axum**, **SurrealDB**, and async functionality.

```toml
[dependencies]
axum = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.28", features = ["full"] }
hyper = { version = "0.14", features = ["server"] }
surrealdb = "1.0.0-beta.9"
dotenv = "0.15"
tokio = { version = "1", features = ["full"] }
```

---

### **Step 2: Create the Employee Schema**

First, create a struct for the **employee details**. We'll use this for serialization/deserialization.

#### **2.1 Define Employee Struct**

```rust
use serde::{Deserialize, Serialize};

// Define Employee Struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: Option<String>, // Optional, as it will be assigned by SurrealDB
    pub name: String,
    pub department: String,
    pub position: String,
    pub salary: f64,
}
```

---

### **Step 3: Connect to SurrealDB**

In the main function, you’ll need to connect to **SurrealDB**.

#### **3.1 SurrealDB Connection**

```rust
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb::sql::Value;
use axum::extract::State;
use std::sync::Arc;

// Function to establish connection
async fn connect_db() -> Surreal<Client> {
    let db = Surreal::new::<Client>("localhost:8000").await.unwrap();
    db.signin("root", "password").await.unwrap();
    db.use_ns("test_namespace").use_db("test_database").await.unwrap();
    db
}

// Application State to hold the DB connection
pub struct AppState {
    pub db: Arc<Surreal<Client>>,
}
```

---

### **Step 4: Implement CRUD Operations**

Next, create the CRUD operations for employee management. These will be tied to HTTP routes handled by Axum.

#### **4.1 Create Employee (POST)**

```rust
use axum::{Json, response::IntoResponse, http::StatusCode};

// Create Employee Handler
pub async fn create_employee(
    State(state): State<AppState>,
    Json(payload): Json<Employee>,
) -> impl IntoResponse {
    let db = &state.db;

    let employee_id = format!("employee:{}", uuid::Uuid::new_v4()); // Generate a unique ID
    let employee = Employee {
        id: Some(employee_id.clone()),
        ..payload
    };

    // Insert employee into SurrealDB
    db.create(&employee_id)
        .content(employee.clone())
        .await
        .unwrap();

    (StatusCode::CREATED, Json(employee))
}
```

#### **4.2 Read Employee by ID (GET)**

```rust
use axum::{extract::Path, response::IntoResponse};

// Read Employee by ID
pub async fn get_employee(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = &state.db;

    let employee: Option<Employee> = db.select(("employee", id)).await.unwrap();

    match employee {
        Some(employee) => (StatusCode::OK, Json(employee)),
        None => (StatusCode::NOT_FOUND, Json("Employee not found")),
    }
}
```

#### **4.3 Update Employee by ID (PUT)**

```rust
use axum::extract::Path;

// Update Employee Handler
pub async fn update_employee(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<Employee>,
) -> impl IntoResponse {
    let db = &state.db;

    let updated_employee = Employee {
        id: Some(id.clone()),
        ..payload
    };

    db.update(("employee", id))
        .content(updated_employee.clone())
        .await
        .unwrap();

    (StatusCode::OK, Json(updated_employee))
}
```

#### **4.4 Delete Employee by ID (DELETE)**

```rust
// Delete Employee Handler
pub async fn delete_employee(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = &state.db;

    db.delete(("employee", id)).await.unwrap();

    (StatusCode::NO_CONTENT)
}
```

---

### **Step 5: Build the Axum Routes**

Now, we can tie these CRUD operations to HTTP routes in Axum.

#### **5.1 Define Axum Routes**

```rust
use axum::{Router, routing::{post, get, put, delete}};
use std::sync::Arc;
use axum::extract::State;

#[tokio::main]
async fn main() {
    let db = connect_db().await;

    // Share the DB connection across the application
    let app_state = AppState {
        db: Arc::new(db),
    };

    let app = Router::new()
        .route("/employees", post(create_employee))
        .route("/employees/:id", get(get_employee))
        .route("/employees/:id", put(update_employee))
        .route("/employees/:id", delete(delete_employee))
        .with_state(app_state);

    // Run the server on port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

### **Step 6: Testing the CRUD Operations**

You can now test the API using `curl` or a tool like **Postman**.

#### **6.1 Create Employee (POST)**

```bash
curl -X POST http://localhost:3000/employees \
    -H 'Content-Type: application/json' \
    -d '{"name": "John Doe", "department": "HR", "position": "Manager", "salary": 60000}'
```

#### **6.2 Get Employee (GET)**

```bash
curl -X GET http://localhost:3000/employees/{employee_id}
```

#### **6.3 Update Employee (PUT)**

```bash
curl -X PUT http://localhost:3000/employees/{employee_id} \
    -H 'Content-Type: application/json' \
    -d '{"name": "John Doe", "department": "HR", "position": "Senior Manager", "salary": 70000}'
```

#### **6.4 Delete Employee (DELETE)**

```bash
curl -X DELETE http://localhost:3000/employees/{employee_id}
```

---

### **Conclusion**

This example demonstrates a basic **CRUD** implementation for managing employee data using **SurrealDB** and **Axum**. You can extend this by adding authentication, validation, or even more complex query functionality based on your project needs.

Yes, **SurrealDB** supports storing vectors, and you can perform operations like embedding search when combined with other tools such as **Candle** for machine learning in Rust.

### **Overview**:

1. **SurrealDB** can store multidimensional vectors in documents.
2. **Candle** (a Rust-based tensor library) can be used to perform operations like generating embeddings from models.
3. You can store these embeddings in SurrealDB and perform similarity searches or filtering operations directly from the database.

### **Steps and Examples**:

1. **Install SurrealDB and Candle**:
   Add the necessary dependencies to your `Cargo.toml`:
   
   ```toml
   [dependencies]
   surrealdb = "0.9"
   candle = "0.3"
   candle-tch = "0.3"
   tokio = { version = "1", features = ["full"] }
   serde_json = "1.0"
   ```
   
   `candle-tch` provides the backend for tensor computations.

2. **Generating Embeddings with Candle**:
   You can use Candle to generate vectors (e.g., from a neural network model) and store those vectors as embeddings in SurrealDB.
   
   Example for generating and storing embeddings:
   
   ```rust
   use candle::{Tensor, Device};
   use candle_tch::TchBackend;
   use surrealdb::Surreal;
   use surrealdb::engine::remote::ws::Client;
   use surrealdb::opt::auth::Root;
   use serde_json::json;
   use tokio;
   
   #[tokio::main]
   async fn main() -> surrealdb::Result<()> {
       // 1. Connect to SurrealDB
       let db = Surreal::new::<Client>("ws://localhost:8000").await?;
   
       // 2. Authenticate
       db.signin(Root {
           username: "root",
           password: "root",
       }).await?;
   
       // 3. Set the namespace and database
       db.use_ns("test_namespace").use_db("test_db").await?;
   
       // 4. Generate a sample embedding using Candle
       let device = Device::cuda_if_available(); // Use CUDA if available, else CPU
       let embedding = Tensor::from_slice(&[0.1, 0.2, 0.3, 0.4], &device)?;
   
       // 5. Convert the embedding to a Vec for storage
       let embedding_vec: Vec<f32> = embedding.to_vec1()?;
   
       // 6. Insert the embedding into SurrealDB
       let response = db.create("embedding")
           .content(json!({
               "name": "sample_embedding",
               "vector": embedding_vec,
           }))
           .await?;
   
       println!("Inserted embedding: {:?}", response);
   
       Ok(())
   }
   ```
   
   - **Candle** is used here to create an embedding (4D vector in this case).
   - The embedding is then stored in **SurrealDB** in a document format.

3. **Embedding Search**:
   SurrealDB doesn’t have native vector search (like FAISS or Milvus), but you can perform similarity searches by calculating the **Euclidean distance** or **cosine similarity** between stored vectors and a query vector, either within the database (with custom SQL) or via Rust code.

4. **Performing a Similarity Search**:
   Suppose you want to retrieve embeddings similar to a given input vector from SurrealDB. You can do this by comparing stored embeddings using cosine similarity or Euclidean distance in your Rust program.
   
   Example to retrieve similar embeddings:
   
   ```rust
   use candle::{Tensor, Device};
   use surrealdb::Surreal;
   use surrealdb::engine::remote::ws::Client;
   use surrealdb::opt::auth::Root;
   use serde_json::Value;
   use tokio;
   
   #[tokio::main]
   async fn main() -> surrealdb::Result<()> {
       // 1. Connect to SurrealDB
       let db = Surreal::new::<Client>("ws://localhost:8000").await?;
   
       // 2. Authenticate
       db.signin(Root {
           username: "root",
           password: "root",
       }).await?;
   
       // 3. Set the namespace and database
       db.use_ns("test_namespace").use_db("test_db").await?;
   
       // 4. Define the input query vector (from which we want to find similar vectors)
       let query_embedding = Tensor::from_slice(&[0.1, 0.2, 0.3, 0.4], &Device::cuda_if_available())?;
   
       // 5. Query all stored embeddings from SurrealDB
       let embeddings: Vec<Value> = db.select("embedding").await?;
   
       // 6. Iterate over embeddings and calculate similarity
       for embed in embeddings {
           if let Some(vector) = embed["vector"].as_array() {
               // Convert the JSON array to a Rust Vec<f32>
               let stored_vec: Vec<f32> = vector.iter().map(|v| v.as_f64().unwrap() as f32).collect();
               let stored_tensor = Tensor::from_slice(&stored_vec, &Device::cuda_if_available())?;
   
               // Calculate Euclidean distance or cosine similarity
               let similarity = cosine_similarity(&query_embedding, &stored_tensor)?;
               println!("Similarity with {:?}: {}", embed["name"], similarity);
           }
       }
   
       Ok(())
   }
   
   fn cosine_similarity(a: &Tensor, b: &Tensor) -> candle::Result<f32> {
       let dot_product = a.dot(b)?;
       let norm_a = a.norm()?;
       let norm_b = b.norm()?;
       let similarity = dot_product.to_scalar::<f32>()? / (norm_a * norm_b).to_scalar::<f32>()?;
       Ok(similarity)
   }
   ```
   
   - This code retrieves all stored embeddings from SurrealDB and calculates cosine similarity with the query embedding.
   - The function `cosine_similarity` compares the embeddings to find the closest match.

### **SurrealDB and Embedding Search in Rust**:

In this example, the embeddings generated by **Candle** are stored in **SurrealDB**, and a simple similarity search is performed using **Rust**. 

In real-world use cases:

- You can scale the embedding search using custom logic.
- For more advanced features (like approximate nearest neighbor searches), you might need specialized vector databases, but for smaller-scale systems, this solution works.

### **Conclusion**:

- **SurrealDB** supports storing vectors and embeddings, and you can use **Candle** in Rust to generate these vectors.
- You can perform similarity search operations like cosine similarity between stored embeddings.

Here’s how to **install SurrealDB** on Linux, start interacting with it, and use examples to get you going with the database. I’ll cover installation steps, starting the database, and performing basic operations like creating, querying, updating, and deleting data.

### **Steps to Install and Set Up SurrealDB on Linux**

1. **Install SurrealDB**:
   SurrealDB is distributed as a single binary, which makes it easy to install.
   
   ```bash
   # Download the latest SurrealDB release
   wget https://github.com/surrealdb/surrealdb/releases/download/1.0.0-beta.10/surreal-linux-x86_64.tar.gz
   
   # Extract the downloaded archive
   tar -xvf surreal-linux-x86_64.tar.gz
   
   # Move the binary to /usr/local/bin
   sudo mv surreal /usr/local/bin/surreal
   
   # Ensure it's executable
   sudo chmod +x /usr/local/bin/surreal
   ```

2. **Start SurrealDB**:
   You can run SurrealDB either in **in-memory** mode for fast prototyping or use **file storage** for persistence.
   
   - **In-Memory Mode**:
     
     ```bash
     surreal start --log debug --user root --pass root memory
     ```
   
   - **File Storage Mode** (for persistence):
     
     ```bash
     surreal start --log debug --user root --pass root file ./surrealdb_data
     ```
   
   This command starts SurrealDB with the user `root` and password `root`, using the filesystem for data storage in `./surrealdb_data`.

3. **Install SurrealDB CLI (Optional)**:
   If you want to interact with SurrealDB via the command line interface (CLI), the same binary is used for both the database server and the CLI.
   
   ```bash
   # Start CLI
   surreal sql --conn http://localhost:8000 --user root --pass root
   ```
   
   This connects to a running SurrealDB instance and provides an SQL-like interface.

### **Running SurrealDB Server with HTTP API**:

SurrealDB provides an HTTP API that allows you to interact with it using REST-like commands, WebSockets, or from a programming language like **Rust**, **JavaScript**, or **Python**.

### **Basic Commands for SurrealDB CLI**

After connecting to the server using `surreal sql`, you can run SQL-like commands to create and manage data:

1. **Select a Namespace and Database**:
   Before creating or querying data, select the namespace and database you want to work with:
   
   ```sql
   USE NS test_namespace DB test_db;
   ```

2. **Create a Table and Insert Data**:
   
   ```sql
   -- Creating a user table and inserting a record
   CREATE user SET name = "John Doe", email = "john@example.com", age = 30;
   ```

3. **Query Data**:
   
   ```sql
   -- Retrieve all users
   SELECT * FROM user;
   
   -- Retrieve a specific user by ID
   SELECT * FROM user WHERE id = user:12345;
   ```

4. **Update Data**:
   
   ```sql
   -- Update a user’s age
   UPDATE user:12345 SET age = 31;
   ```

5. **Delete Data**:
   
   ```sql
   -- Delete a user by ID
   DELETE user:12345;
   ```

### **Interacting with SurrealDB in Rust**

You can use SurrealDB with **Rust** by adding the `surrealdb` crate to your project.

1. **Add Dependencies to `Cargo.toml`**:
   
   ```toml
   [dependencies]
   surrealdb = "0.9"
   tokio = { version = "1", features = ["full"] }
   serde_json = "1.0"
   ```

2. **Example Code in Rust**:
   
   ```rust
   use surrealdb::Surreal;
   use surrealdb::engine::remote::ws::Client;
   use surrealdb::opt::auth::Root;
   use tokio;
   
   #[tokio::main]
   async fn main() -> surrealdb::Result<()> {
       // Connect to SurrealDB server
       let db = Surreal::new::<Client>("ws://localhost:8000").await?;
   
       // Sign in using root credentials
       db.signin(Root {
           username: "root",
           password: "root",
       }).await?;
   
       // Select namespace and database
       db.use_ns("test_namespace").use_db("test_db").await?;
   
       // Insert data into the 'user' table
       let user = db.create("user")
           .content(serde_json::json!({
               "name": "Alice",
               "email": "alice@example.com",
               "age": 28
           }))
           .await?;
   
       println!("Inserted User: {:?}", user);
   
       // Query the 'user' table for all records
       let users: serde_json::Value = db.select("user").await?;
       println!("Users: {:?}", users);
   
       // Update a user's age
       let updated_user = db.update(("user", "alice"))
           .content(serde_json::json!({
               "age": 29
           }))
           .await?;
   
       println!("Updated User: {:?}", updated_user);
   
       // Delete the user
       db.delete(("user", "alice")).await?;
   
       Ok(())
   }
   ```

3. **Explanation**:
   
   - **Connection**: The client connects to SurrealDB using WebSockets (`ws://localhost:8000`).
   - **Operations**: It performs CRUD operations (create, read, update, and delete).
   - **Namespace and Database**: It uses the `test_namespace` and `test_db` to keep operations scoped.

### **Real-time Interaction with WebSockets**

SurrealDB supports real-time data changes using WebSockets, which is very useful for building real-time applications like chat apps, IoT dashboards, etc.

1. **Subscribing to Changes**:
   You can subscribe to changes to any table or document. When the data changes, you receive real-time updates.
   
   Example (in Rust):
   
   ```rust
   use surrealdb::Surreal;
   use surrealdb::engine::remote::ws::Client;
   use surrealdb::opt::auth::Root;
   use tokio;
   
   #[tokio::main]
   async fn main() -> surrealdb::Result<()> {
       // Connect to SurrealDB
       let db = Surreal::new::<Client>("ws://localhost:8000").await?;
   
       // Authenticate
       db.signin(Root {
           username: "root",
           password: "root",
       }).await?;
   
       // Subscribe to changes in the 'user' table
       let mut sub = db.subscribe("user").await?;
   
       // Print any changes as they occur
       while let Some(change) = sub.next().await {
           println!("Change detected: {:?}", change);
       }
   
       Ok(())
   }
   ```
   
   In this example, the program subscribes to the `user` table, and any change (insert, update, or delete) will be printed to the console.

### **Running SurrealDB with Docker**:

If you prefer using Docker, here's how you can run SurrealDB in a Docker container:

1. **Pull the Docker Image**:
   
   ```bash
   docker pull surrealdb/surrealdb:latest
   ```

2. **Run the Docker Container**:
   
   ```bash
   docker run --rm -p 8000:8000 surrealdb/surrealdb:latest start --user root --pass root
   ```

3. **Access the SurrealDB Instance**:
   You can now interact with the running SurrealDB server using the Rust client or any other client.

---

### **Conclusion**:

- **SurrealDB** provides a flexible, modern, and efficient way to manage both document and graph data models. 
- **Real-time capabilities** and **multi-model support** make it a strong choice for modern applications.
- It can be integrated with Rust, JavaScript, and other languages for seamless application development.

By following the above steps, you'll have SurrealDB installed, running, and ready for development on your Linux machine.

The error you're encountering occurs because SurrealDB expects a specific type for the connection endpoint, and the type provided doesn't match the expected one. The `IntoEndpoint` trait in SurrealDB defines which types can be used as endpoints for different connection schemes (e.g., WebSocket `ws`, secure WebSocket `wss`).

Here's how you can resolve it:

### Correct Code Snippet:

If you're using **WebSocket (`ws`)** as the protocol, make sure to use `surrealdb::engine::remote::ws::Ws` or `surrealdb::engine::remote::ws::Wss` for **secure WebSocket (`wss`)**.

```rust
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

async fn establish_connection() -> surrealdb::Result<Client> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: "root",
        password: "password",
    })
    .await?;

    db.use_ns("test_ns").use_db("test_db").await?;
    Ok(db)
}
```

Here's a basic example of an Axum server that uses shared state and implements a POST request handler. The state holds a simple counter, and the POST request increments it.

### Code Example

```rust
use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use std::net::SocketAddr;

// Shared state struct
#[derive(Clone)]
struct AppState {
    counter: Arc<AsyncMutex<i32>>, // Shared state with an async Mutex for thread safety
}

// Data received from the POST request
#[derive(Deserialize)]
struct IncrementPayload {
    amount: i32,
}

// POST request handler
async fn increment_counter(
    State(state): State<AppState>, 
    Json(payload): Json<IncrementPayload>
) -> (StatusCode, String) {
    let mut counter = state.counter.lock().await; // Lock the async mutex and access the counter
    *counter += payload.amount;

    (
        StatusCode::OK,
        format!("Counter updated to: {}", *counter),
    )
}

#[tokio::main]
async fn main() {
    // Initialize shared state with a counter value of 0
    let state = AppState {
        counter: Arc::new(AsyncMutex::new(0)),
    };

    // Define routes
    let app = Router::new()
        .route("/increment", post(increment_counter))
        .with_state(state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Explanation:

1. **State (`AppState`)**: Holds a counter inside an async `Mutex` for thread-safe access in an async environment.
2. **Handler (`increment_counter`)**: This function receives the state and the POST payload, increments the counter, and returns a response.
3. **Server (`axum::Server`)**: Binds to `127.0.0.1:3000` and listens for incoming POST requests at `/increment`.

### Example POST Request:

You can send a POST request using `curl`:

```bash
curl -X POST http://localhost:3000/increment -H "Content-Type: application/json" -d '{"amount": 5}'
```

This will increment the counter by 5 and return the updated counter value.
