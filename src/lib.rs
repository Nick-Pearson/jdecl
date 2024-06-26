use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn explain(input: &str) -> String {
    match parse(input) {
        Ok(parsed) => parsed
            .iter()
            .rev()
            .map(explain_location)
            .collect::<Vec<String>>()
            .join(", "),
        Err(_) => "Invalid input".to_string(),
    }
}

fn explain_location(location: &LocationKind) -> String {
    match location {
        LocationKind::Package { name } => {
            format!("in package {}", name)
        }
        LocationKind::Class { name } => {
            format!("part of class {}", name)
        }
        LocationKind::Method { name } => {
            format!("inside method {}", name)
        }
        LocationKind::Lambda { name, id } => {
            format!("The {} lambda created inside method {}", nth(id), name)
        }
    }
}

fn nth(n: &u32) -> String {
    match n {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", n),
    }
}

#[derive(Debug, PartialEq)]
enum LocationKind<'a> {
    Package { name: &'a str },
    Class { name: &'a str },
    // InnerClass { name: &'a str },
    Method { name: &'a str },
    Lambda { name: &'a str, id: u32 },
}

#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn parse(input: &str) -> Result<Vec<LocationKind>, Error> {
    let file_idx = input.rfind('(').ok_or(Error::InvalidInput)?;

    let path = &input[..file_idx];
    let method_idx = path.rfind('.').ok_or(Error::InvalidInput)?;
    let class_idx = path[..method_idx].rfind('.').ok_or(Error::InvalidInput)?;

    let mut result = Vec::new();
    result.push(LocationKind::Package {
        name: &path[..class_idx],
    });
    result.push(LocationKind::Class {
        name: &path[class_idx + 1..method_idx],
    });

    let method = &path[method_idx + 1..];

    if method.starts_with("lambda$") {
        let lambda_idx = method.find('$').ok_or(Error::InvalidInput)?;
        let id_idx = method.rfind('$').ok_or(Error::InvalidInput)?;

        let id = &method[id_idx + 1..];
        let name = &method[lambda_idx + 1..id_idx];
        result.push(LocationKind::Lambda {
            name,
            id: id.parse().unwrap(),
        });
    } else {
        result.push(LocationKind::Method { name: method });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method() {
        let input = "org.jdecl.parser.InputTextParser.getInputHandler(InputTextParser.java:425)";
        let parsed = parse(input).unwrap();

        let expected = vec![
            LocationKind::Package {
                name: "org.jdecl.parser",
            },
            LocationKind::Class {
                name: "InputTextParser",
            },
            LocationKind::Method {
                name: "getInputHandler",
            },
        ];
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_parse_lambda() {
        let input =
            "org.jdecl.parser.InputTextParser.lambda$getInputHandler$2(InputTextParser.java:425)";
        let parsed = parse(input).unwrap();

        let expected = vec![
            LocationKind::Package {
                name: "org.jdecl.parser",
            },
            LocationKind::Class {
                name: "InputTextParser",
            },
            LocationKind::Lambda {
                name: "getInputHandler",
                id: 2,
            },
        ];
        assert_eq!(expected, parsed);
    }
}
