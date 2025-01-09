use reqwest::blocking::Client;
use reqwest::Error;


fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "Victor".to_string();
    let password:Option<String> = None;
    let request_url = "https://httpbin.org/get";

    let response = client
        .get(request_url)
        .basic_auth(user, password)
        .send();
    println!("{:?}", response);



    Ok(())
}