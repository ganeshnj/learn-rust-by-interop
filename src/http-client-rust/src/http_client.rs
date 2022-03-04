use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::time::Duration;

pub  trait Callable {
    fn get(&self, request: &HttpRequest) -> Result<HttpResponse, String>;
}

pub struct HttpClient {
}

#[derive(Debug)]
pub struct HttpRequest {
    pub host: String,
    pub path: String,
    pub headers: HashMap<String, String>
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: String,
    pub body: String,
    pub headers: HashMap<String, String>
}

impl Callable for HttpClient {
    fn get(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        if let mut stream = TcpStream::connect(request.host.as_str()).unwrap() {
            println!("Connected to {}", request.host);

            stream.set_read_timeout(Some(Duration::new(5, 0))).unwrap();
            stream.set_write_timeout(Some(Duration::new(5, 0))).unwrap();

            let mut request_string = String::new();
            request_string.push_str("GET ");
            request_string.push_str(request.path.as_str());
            request_string.push_str(" HTTP/1.1\r\n");
            request_string.push_str("Host: ");
            request_string.push_str(request.host.as_str());
            request_string.push_str("\r\n");
            request_string.push_str("Connection: close\r\n");

            for (key, value) in request.headers.iter() {
                request_string.push_str(key.as_str());
                request_string.push_str(": ");
                request_string.push_str(value.as_str());
                request_string.push_str("\r\n");
            }

            request_string.push_str("\r\n");
            println!("Request: {}", request_string);

            let write_size = stream.write(request_string.as_bytes()).unwrap();
            println!("Wrote {} bytes", write_size);

            let mut response_string = String::new();
            let read_size = stream.read_to_string(&mut response_string).unwrap();
            println!("Read {} bytes", read_size);

            let mut status = String::new();
            let mut headers = HashMap::new();
            let mut body = String::new();
            let mut lines = response_string.lines();
            status.push_str(lines.next().unwrap());

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                let mut parts = line.split(": ");
                let key = parts.next().unwrap();
                let value = parts.next().unwrap();
                headers.insert(key.to_string(), value.to_string());
            }

            while let Some(line) = lines.next() {
                // some weird chars are being read here
                if line.starts_with("1") || line == "0" {
                    continue;
                }
                body.push_str(line);
                body.push_str("\n");
            }

            Ok(HttpResponse {
                status,
                headers,
                body
            })
        } else {
            return Err(format!("Could not connect to {}", request.host));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::http_client::{Callable, HttpClient, HttpRequest};

    #[test]
    fn http_client_get() {
        let host = "localhost:5104";
        let path = "/WeatherForecast";

        let client = HttpClient {};
        let response = client.get(&HttpRequest {
            host: host.to_string(),
            path: path.to_string(),
            headers: HashMap::new()
        }).unwrap();
        assert_eq!(response.status, "HTTP/1.1 200 OK");
    }
}
