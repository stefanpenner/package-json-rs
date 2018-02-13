#[macro_use]
extern crate indoc;
extern crate serde_json;
extern crate serde;
use serde::ser;
use serde_json::{Value, Error};

pub fn something(pkg: &'static str) ->  Result<(), Error>
{
    // println!("{}", pkg);
    // Parse the string of data into serde_json::Value.
    let value: Value = serde_json::from_str(pkg)?;

    // Access parts of the data by indexing with square brackets.
    // println!("Please call {} at the number {}", value["name"], value["phones"][0]);
    println!("-------------");
    // println!("{}", serde_json::to_string(&value)?);
    println!("-------------");
    // println!("{}", serde_json::to_string_pretty(&value)?);

    let mut writer = Vec::with_capacity(128);
    let mut serializer = serde_json::Serializer::with_formatter(writer, serde_json::ser::PrettyFormatter::with_indent(b"  "));

    value.serialize(&mut serializer);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pkg = indoc!(r#"
          {
            "name": "asdfasdf",
            "version": "1.0.0",
            "description": "",
            "author": "",
            "main": "index.js",
            "license": "ISC"
          }
        "#);

        something(pkg);
    }
}
