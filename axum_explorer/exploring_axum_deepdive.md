### Exploring Axum Deep Dive

Most Used dependencies:

- axum | serde | tokio | 

- [x] Complete the basic exercise
  
  - [x] [Axum Documentation](https://programatik29.github.io/axum-tutorial/#/01-introduction)

- [x] Start by exploring the  Documentation
  
  > - [x] Explore the Extractors
  >   
  >   - Working with provided documentation failed. Learnt about debug_handler 
  >   
  >   - Due to some dep issue, the Deserializer acted weird
  > 
  > - [ ] Explore the Responses
  > 
  > - [ ] Explore the Middle Wares

- [x] Explore the other examples from the community
  
  - [x] [GitHub - joelparkerhenderson/demo-rust-axum: Demo of Rust and axum web framework with Tokio, Tower, Hyper, Serde](https://github.com/joelparkerhenderson/demo-rust-axum)
    
    - [x] Create a simple hello world server with closure handler
    
    - [x] Create a handler that returns a String
    
    - [x] Implement fallback. Need to use Uri type, and the response must impl auxm::response::IntoResponse trait
    
    - [x] Practice on different response ranging from Html text to HTTP verbs
      
      - [x] simple html text
      
      - [x] html file like index.html
      
      - [x] sending demo status
      
      - [x] returning the uri and printing it
      
      - [x] fallback implementation
      
      - [x] updating headers & send png
      
      - [x] Work with get/post/put/patch/delete
    
    - [x] Continue working with Extractors
      
      - [x] Path to extract id
      
      - [x] Query  extract in HashMap
      
      - [x] Query extract in Deserialized Struct
      
      - [x] Query extract json body from put / post 
    
    - [x] Work on Json response
      
      - [x] Build Response with serde_json's json and Value
    
    - [x] Work on implementing the Book Maker Restful API

- [x] Work on writing the reqwest for Book Maker
  
  - Moved the book_reqwester outside of the axum_explorer workspace
  
  - facing issue with build hanging at mime v0.3.17:
    
    - realized that open_ssl was doing a lot more configuration
    
    - use cargo build -vv to understand that build was happening under the hood
  
  - [x] writing simple get request directly over reqwest
  
  - [x] spawning a client and then posting the data to serve

- [x] Work on SQLx or Diesel with Axum. (parking it)
  
  - Diesel is the ORM and the Query Builder
  
  - Working with ORM has not much work with Candle  
