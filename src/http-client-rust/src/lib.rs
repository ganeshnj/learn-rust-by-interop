use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::http_client::{Callable, HttpClient, HttpRequest};

mod http_client;

#[no_mangle]
pub extern fn http_client_get(request: &CHttpRequest) -> CHttpResponse {
    let client = HttpClient {};
    let host: String = unsafe { CStr::from_ptr(request.host) }.to_str().expect("Can not read host argument.").to_string();
    let path: String = unsafe { CStr::from_ptr(request.path) }.to_str().expect("Can not read path argument.").to_string();
    let response = client.get(&HttpRequest {
        host,
        headers: HashMap::new(),
        path
    }).unwrap();
    CHttpResponse {
        status: CString::new(response.status.to_string()).unwrap().into_raw(),
        body: CString::new(response.body.to_string()).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern fn http_client_get_flat(host: *const c_char, path: *const c_char) -> *mut c_char {
    let client = HttpClient {};
    let host: String = unsafe { CStr::from_ptr(host) }.to_str().expect("Can not read host argument.").to_string();
    let path: String = unsafe { CStr::from_ptr(path) }.to_str().expect("Can not read path argument.").to_string();
    let response = client.get(&HttpRequest {
        host,
        headers: HashMap::new(),
        path
    }).unwrap();
    CString::new(response.body.to_string()).unwrap().into_raw()
}

#[repr(C)]
pub struct CHttpResponse {
    pub status: *mut c_char,
    pub body: *mut c_char,
}


#[repr(C)]
pub struct CHttpRequest {
    pub host: *mut c_char,
    pub path: *mut c_char,
}

#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ffi::{CStr, CString};
    use crate::http_client::{Callable, HttpClient, HttpRequest};
    use crate::{CHttpRequest, http_client_get};

    #[test]
    fn test_http_client_get() {
        let request = CHttpRequest {
            host: CString::new("localhost:5104").unwrap().into_raw(),
            path: CString::new("/WeatherForecast").unwrap().into_raw(),
        };

        let response = http_client_get(&request);
        let status = unsafe { CStr::from_ptr(response.status) }.to_str().unwrap();
        assert_eq!(status, "HTTP/1.1 200 OK");
    }
}
