## `supabase-function-rs` Library

### Overview

`supabase-function-rs` is a Rust client library designed to interact with Supabase Edge Functions. This library provides a straightforward way to invoke Supabase functions with various HTTP methods, headers, and body types, including JSON, plain text, form data, file uploads, blobs, and array buffers. It also supports setting custom regions for function invocation.

### Features

- Invoke Supabase Edge Functions with different HTTP methods.
- Support for various body types: JSON, plain text, form data, file uploads, blobs, and array buffers.
- Custom headers support.
- Region-specific function invocation.
- Comprehensive error handling.

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
supabase-function-rs = "0.1.0"
```

### Usage

#### Initialization

To get started, initialize the `FunctionsClient` with the Supabase URL and optional headers and region.

```rust
use supabase_function_rs::{FunctionsClient, FunctionRegion};
use std::collections::HashMap;

let url = "https://your-supabase-url";
let mut headers = HashMap::new();
headers.insert("Custom-Header".to_string(), "Header-Value".to_string());

let client = FunctionsClient::new(url.to_string(), Some(headers), Some(FunctionRegion::UsEast1));
client.set_auth("your-jwt-token".to_string());
```

#### Basic Function Invocation

```rust
use supabase_function_rs::{FunctionInvokeOptions, HttpMethod, InvokeBody, FunctionsResponse, ResponseData};
use serde_json::json;
use std::collections::HashMap;

let mut invoke_options = FunctionInvokeOptions::default();
invoke_options.method = Some(HttpMethod::Post);
let mut json_body = HashMap::new();
json_body.insert("request_key".to_string(), json!("request_value"));
invoke_options.body = Some(InvokeBody::Json(json_body));

match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => match response {
        FunctionsResponse::Success { data } => match data {
            ResponseData::Json(json) => {
                println!("Response: {:?}", json);
            }
            _ => panic!("Expected JSON response data"),
        },
        FunctionsResponse::Failure { error } => {
            println!("Function call failed: {}", error);
        }
    },
    Err(e) => {
        println!("Error invoking function: {}", e);
    }
}
```

#### Function Invocation with Different Body Types

##### Plain Text

```rust
invoke_options.body = Some(InvokeBody::String("request text".to_string()));
match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => { /* Handle response */ },
    Err(e) => { /* Handle error */ }
}
```

##### Form Data

```rust
let mut form_data = HashMap::new();
form_data.insert("field1".to_string(), "value1".to_string());
form_data.insert("field2".to_string(), "value2".to_string());
invoke_options.body = Some(InvokeBody::FormData(form_data));
match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => { /* Handle response */ },
    Err(e) => { /* Handle error */ }
}
```

##### File Upload

```rust
use std::fs::File;
use std::io::Read;

let path = "test_file.txt";
let mut file = File::open(&path).unwrap();
let mut file_content = Vec::new();
file.read_to_end(&mut file_content).unwrap();
invoke_options.body = Some(InvokeBody::File(file_content));
match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => { /* Handle response */ },
    Err(e) => { /* Handle error */ }
}
```

##### Blob

```rust
let blob: Vec<u8> = vec![1, 2, 3, 4, 5]; // Example blob data
invoke_options.body = Some(InvokeBody::Blob(blob));
match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => { /* Handle response */ },
    Err(e) => { /* Handle error */ }
}
```

##### ArrayBuffer

```rust
let array_buffer: Vec<u8> = vec![1, 2, 3, 4, 5]; // Example array buffer data
invoke_options.body = Some(InvokeBody::ArrayBuffer(array_buffer));
match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => { /* Handle response */ },
    Err(e) => { /* Handle error */ }
}
```

#### Setting a Custom Region

You can specify a custom region when invoking a function:

```rust
invoke_options.region = Some(FunctionRegion::UsEast1);
match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => { /* Handle response */ },
    Err(e) => { /* Handle error */ }
}
```

### Error Handling

The library provides comprehensive error handling with specific error types:

- `FunctionsFetchError`: Indicates a failure to send the request.
- `FunctionsRelayError`: Indicates a relay error when invoking the function.
- `FunctionsHttpError`: Indicates a non-2xx status code returned by the function.

Example:

```rust
match client.invoke("function-name", Some(invoke_options)).await {
    Ok(response) => { /* Handle response */ },
    Err(FunctionsError::FetchError(e)) => println!("Fetch error: {}", e),
    Err(FunctionsError::RelayError(e)) => println!("Relay error: {}", e),
    Err(FunctionsError::HttpError(e)) => println!("HTTP error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

### Testing

To run the tests, use:

```sh
cargo test --test functions_client_tests
```

### Contributing

Feel free to open issues or submit pull requests for new features, bug fixes, or improvements.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Acknowledgements

Special thanks to the contributors and the open-source community for their support and contributions.
