use graphql_client::Error;
pub fn map_graphql_errors_to_string(
    errors:&Option<Vec<Error>>
) -> String {
    if let Some(errors) = errors {
        errors
            .iter()
            .fold(
                String::new(),
                |acc, err|
                    format!("{}\n{}", acc, err.message.clone()
                    ))
    } else {
        "Error: Expected graphql error msg not found.".into()
    }
}