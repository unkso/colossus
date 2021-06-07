#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    libapi::rocket().launch().await
}
