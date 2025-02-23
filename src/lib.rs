use extism_pdk::*;
use serde::{Deserialize, Serialize};

fn fact() -> Result<String, Error> {
    #[derive(Serialize, Deserialize)]
    struct Fact {
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
    let redact = config::get("redact")?;
    match (redact, fact()) {
        (Ok(fact), Some(redact)) => Ok(Output {
            name: format!("--{}--", redact),
            age: input.age,
            happy: input.happy,
            fact,
        }),
        (Err(err), _) => Err(WithReturnCode::new(err, 1)),
    }
}
