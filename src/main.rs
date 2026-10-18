
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/books", get(list_books).post(create_book))
        .route(
            "/books/{id}",
            get(get_book)
                .put(replace_book)
                .patch(update_book)
                .delete(delete_book),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Welcome to my Library API!"
}

async fn list_books() -> &'static str {
    "Listing all books"
}

async fn create_book() -> &'static str {
    "Creating a book"
}

async fn get_book() -> &'static str {
    "Getting one book"
}

async fn replace_book() -> &'static str {
    "Replacing a book"
}

async fn update_book() -> &'static str {
    "Updating part of a book"
}

async fn delete_book() -> &'static str {
    "Deleting a book"
}
