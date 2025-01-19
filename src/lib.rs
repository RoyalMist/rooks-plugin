use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug)]
#[encoding(Json)]
struct Data {
    name: String,
    age: i32,
    happy: bool,
}

#[plugin_fn]
pub fn redact(input: Data) -> FnResult<Data> {
    let redact = config::get("redact").expect("redact not found");
    Ok(Data {
        name: redact.unwrap(),
        age: input.age,
        happy: input.happy,
    })
}
