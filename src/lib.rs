use extism_pdk::*;
use serde::{Deserialize, Serialize};

fn fact() -> FnResult<String> {
    #[derive(Serialize, Deserialize)]
    struct Fact {
        pub icon_url: String,
        pub id: String,
        pub url: String,
        pub value: String,
    }

    let request = HttpRequest::new("https://api.chucknorris.io/jokes/random").with_method("GET");
    let res = http::request::<()>(&request, None)?;
    let fact = res.json::<Fact>()?;
    Ok(fact.value)
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug)]
#[encoding(Json)]
struct Input {
    name: String,
    age: i32,
    happy: bool,
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug)]
#[encoding(Json)]
struct Output {
    name: String,
    age: i32,
    happy: bool,
    fact: String,
}

#[plugin_fn]
pub fn call(input: Input) -> FnResult<Output> {
    info!("Hello from Plugin!");
    let redact = config::get("redact").expect("redact not found");

    Ok(Output {
        name: format!("New value: {}", redact.unwrap()),
        age: input.age,
        happy: input.happy,
        fact: fact()?,
    })
}
