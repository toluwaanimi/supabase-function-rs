mod functions_client_tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;
    use mockito::mock;
    use serde_json::json;
    use supabase_function_rs::{
        FunctionInvokeOptions, FunctionsClient, FunctionsResponse, HttpMethod, InvokeBody,
        ResponseData, FunctionRegion,
    };

    #[tokio::test]
    async fn test_invoke_basic() {
        let _m = mock("POST", "/function-name")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let url = &mockito::server_url();
        println!("Mock server URL: {}", url);

        let mut client = FunctionsClient::new(url.to_string(), None, None);
        client.set_auth("test-token".to_string());

        let mut invoke_options = FunctionInvokeOptions::default();
        invoke_options.method = Some(HttpMethod::Post);
        let mut json_body = HashMap::new();
        json_body.insert("request_key".to_string(), json!("request_value"));
        invoke_options.body = Some(InvokeBody::Json(json_body));
        println!("Invoking function with options: {:?}", invoke_options);

        match client.invoke("function-name", Some(invoke_options)).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                match response {
                    FunctionsResponse::Success { data } => match data {
                        ResponseData::Json(json) => {
                            assert_eq!(json["key"], "value");
                        }
                        _ => panic!("Expected JSON response data"),
                    },
                    FunctionsResponse::Failure { error } => {
                        panic!("Expected success, got failure: {}", error);
                    }
                }
            }
            Err(e) => {
                panic!("Error invoking function: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_invoke_with_region() {
        let _m = mock("POST", "/function-name")
            .match_header("x-region", "us-east-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let url = &mockito::server_url();
        println!("Mock server URL: {}", url);

        let mut client = FunctionsClient::new(url.to_string(), None, Some(FunctionRegion::UsEast1));
        client.set_auth("test-token".to_string());

        let mut invoke_options = FunctionInvokeOptions::default();
        invoke_options.method = Some(HttpMethod::Post);
        let mut json_body = HashMap::new();
        json_body.insert("request_key".to_string(), json!("request_value"));
        invoke_options.body = Some(InvokeBody::Json(json_body));
        invoke_options.region = Some(FunctionRegion::UsEast1);
        println!("Invoking function with options: {:?}", invoke_options);

        match client.invoke("function-name", Some(invoke_options)).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                match response {
                    FunctionsResponse::Success { data } => match data {
                        ResponseData::Json(json) => {
                            assert_eq!(json["key"], "value");
                        }
                        _ => panic!("Expected JSON response data"),
                    },
                    FunctionsResponse::Failure { error } => {
                        panic!("Expected success, got failure: {}", error);
                    }
                }
            }
            Err(e) => {
                panic!("Error invoking function: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_invoke_with_text_body() {
        let _m = mock("POST", "/function-name")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let url = &mockito::server_url();
        println!("Mock server URL: {}", url);

        let mut client = FunctionsClient::new(url.to_string(), None, None);
        client.set_auth("test-token".to_string());

        let mut invoke_options = FunctionInvokeOptions::default();
        invoke_options.method = Some(HttpMethod::Post);
        invoke_options.body = Some(InvokeBody::String("request text".to_string()));
        println!("Invoking function with options: {:?}", invoke_options);

        match client.invoke("function-name", Some(invoke_options)).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                match response {
                    FunctionsResponse::Success { data } => match data {
                        ResponseData::Json(json) => {
                            assert_eq!(json["key"], "value");
                        }
                        _ => panic!("Expected JSON response data"),
                    },
                    FunctionsResponse::Failure { error } => {
                        panic!("Expected success, got failure: {}", error);
                    }
                }
            }
            Err(e) => {
                panic!("Error invoking function: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_invoke_with_form_data() {
        let _m = mock("POST", "/function-name")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let url = &mockito::server_url();
        println!("Mock server URL: {}", url);

        let mut client = FunctionsClient::new(url.to_string(), None, None);
        client.set_auth("test-token".to_string());

        let mut invoke_options = FunctionInvokeOptions::default();
        invoke_options.method = Some(HttpMethod::Post);
        let mut form_data = HashMap::new();
        form_data.insert("field1".to_string(), "value1".to_string());
        form_data.insert("field2".to_string(), "value2".to_string());
        invoke_options.body = Some(InvokeBody::FormData(form_data));
        println!("Invoking function with options: {:?}", invoke_options);

        match client.invoke("function-name", Some(invoke_options)).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                match response {
                    FunctionsResponse::Success { data } => match data {
                        ResponseData::Json(json) => {
                            assert_eq!(json["key"], "value");
                        }
                        _ => panic!("Expected JSON response data"),
                    },
                    FunctionsResponse::Failure { error } => {
                        panic!("Expected success, got failure: {}", error);
                    }
                }
            }
            Err(e) => {
                panic!("Error invoking function: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_invoke_with_file() {
        let _m = mock("POST", "/function-name")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let url = &mockito::server_url();
        println!("Mock server URL: {}", url);

        let mut client = FunctionsClient::new(url.to_string(), None, None);
        client.set_auth("test-token".to_string());

        let mut invoke_options = FunctionInvokeOptions::default();
        invoke_options.method = Some(HttpMethod::Post);

        // Create a temporary file for testing
        let path = Path::new("test_file.txt");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "This is a test file").unwrap();

        // Read file contents into Vec<u8>
        let mut file_content = Vec::new();
        let mut file = File::open(&path).unwrap();
        file.read_to_end(&mut file_content).unwrap();

        invoke_options.body = Some(InvokeBody::File(file_content));
        println!("Invoking function with options: {:?}", invoke_options);

        match client.invoke("function-name", Some(invoke_options)).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                match response {
                    FunctionsResponse::Success { data } => match data {
                        ResponseData::Json(json) => {
                            assert_eq!(json["key"], "value");
                        }
                        _ => panic!("Expected JSON response data"),
                    },
                    FunctionsResponse::Failure { error } => {
                        panic!("Expected success, got failure: {}", error);
                    }
                }
            }
            Err(e) => {
                panic!("Error invoking function: {}", e);
            }
        }

        // Clean up the temporary file
        std::fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn test_invoke_with_blob() {
        let _m = mock("POST", "/function-name")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let url = &mockito::server_url();
        println!("Mock server URL: {}", url);

        let mut client = FunctionsClient::new(url.to_string(), None, None);
        client.set_auth("test-token".to_string());

        let mut invoke_options = FunctionInvokeOptions::default();
        invoke_options.method = Some(HttpMethod::Post);

        let blob: Vec<u8> = vec![1, 2, 3, 4, 5]; // Example blob data
        invoke_options.body = Some(InvokeBody::Blob(blob));
        println!("Invoking function with options: {:?}", invoke_options);

        match client.invoke("function-name", Some(invoke_options)).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                match response {
                    FunctionsResponse::Success { data } => match data {
                        ResponseData::Json(json) => {
                            assert_eq!(json["key"], "value");
                        }
                        _ => panic!("Expected JSON response data"),
                    },
                    FunctionsResponse::Failure { error } => {
                        panic!("Expected success, got failure: {}", error);
                    }
                }
            }
            Err(e) => {
                panic!("Error invoking function: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_invoke_with_array_buffer() {
        let _m = mock("POST", "/function-name")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let url = &mockito::server_url();
        println!("Mock server URL: {}", url);

        let mut client = FunctionsClient::new(url.to_string(), None, None);
        client.set_auth("test-token".to_string());

        let mut invoke_options = FunctionInvokeOptions::default();
        invoke_options.method = Some(HttpMethod::Post);

        let array_buffer: Vec<u8> = vec![1, 2, 3, 4, 5]; // Example array buffer data
        invoke_options.body = Some(InvokeBody::ArrayBuffer(array_buffer));
        println!("Invoking function with options: {:?}", invoke_options);

        match client.invoke("function-name", Some(invoke_options)).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                match response {
                    FunctionsResponse::Success { data } => match data {
                        ResponseData::Json(json) => {
                            assert_eq!(json["key"], "value");
                        }
                        _ => panic!("Expected JSON response data"),
                    },
                    FunctionsResponse::Failure { error } => {
                        panic!("Expected success, got failure: {}", error);
                    }
                }
            }
            Err(e) => {
                panic!("Error invoking function: {}", e);
            }
        }
    }
}
