//
//  main.swift
//  console-app
//
//  Created by Ganesh Jangir on 3/3/22.
//

import Foundation

print("Hello, World!")

struct HttpRequest {
    let host: String
    let path: String
}

struct HttpResponse {
    let status: String
    let body: String
}

class HttpClient {
    func get(request: HttpRequest) -> String {
        let cBody = http_client_get_flat(request.host, request.path);
        let body = String(cString: cBody!)
        return body
    }
}

let client = HttpClient()
let request = HttpRequest(host: "localhost:5104", path: "/WeatherForecast")
let body = client.get(request: request)

print(body)
