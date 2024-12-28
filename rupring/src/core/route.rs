use hyper::Method;

pub(crate) fn is_route_matching_request(route_path: String, request_path: &str) -> bool {
    // remove query parameters
    let request_path = request_path.split("?").collect::<Vec<&str>>()[0];

    let route_path = route_path.split("/").collect::<Vec<&str>>();
    let request_path = request_path.split("/").collect::<Vec<&str>>();

    if route_path.len() != request_path.len() {
        return false;
    }

    for (route_path_part, request_path_part) in route_path.iter().zip(request_path.iter()) {
        if route_path_part.starts_with(":") {
            continue;
        }

        if route_path_part != request_path_part {
            return false;
        }
    }

     true
}

pub(crate) fn normalize_path(prefix: String, path: String) -> String {
    let mut normalized_path = "/".to_string();

    if prefix.starts_with("/") {
        normalized_path.push_str(&prefix[1..]);
    } else {
        normalized_path.push_str(&prefix);
    }

    if !normalized_path.ends_with("/") {
        normalized_path.push_str("/");
    }

    if path.starts_with("/") {
        normalized_path.push_str(&path[1..]);
    } else {
        normalized_path.push_str(&path);
    }

    if normalized_path.ends_with("/") && normalized_path.len() > 1 {
        normalized_path.pop();
    }

     normalized_path
}

// return (route, route_path, middlewares)
pub(crate) fn find_route(
    root_module: Box<dyn crate::IModule>,
    request_path: &str,
    request_method: &Method,
) -> Option<(
    Box<dyn crate::IRoute + Send + 'static>,
    String,
    Vec<crate::MiddlewareFunction>,
)> {
    for controller in root_module.controllers() {
        let prefix = controller.prefix();

        for route in controller.routes() {
            if route.method() != request_method {
                continue;
            }

            let route_path = normalize_path(prefix.clone(), route.path());

            if !is_route_matching_request(route_path.clone(), request_path) {
                continue;
            }

            let middlewares = root_module
                .middlewares()
                .into_iter()
                .chain(controller.middlewares().into_iter())
                .collect();
            return Some((route, route_path, middlewares));
        }
    }

    for child_module in root_module.child_modules() {
        let result = find_route(child_module, request_path, request_method);

        match result {
            Some((route, route_path, middlewares)) => {
                let middlewares = root_module
                    .middlewares()
                    .into_iter()
                    .chain(middlewares.into_iter())
                    .collect();
                return Some((route, route_path, middlewares));
            }
            None => {}
        }
    }

     None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_route_matching_request() {
        struct TestCase {
            name: String,
            route_path: String,
            request_path: String,
            expected: bool,
        }

        let test_cases = vec![
            TestCase {
                name: "route_path is empty".to_string(),
                route_path: "".to_string(),
                request_path: "".to_string(),
                expected: true,
            },
            TestCase {
                name: "route_path and request_path are the same".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello".to_string(),
                expected: true,
            },
            TestCase {
                name: "route_path and request_path are different".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/world".to_string(),
                expected: false,
            },
            TestCase {
                name: "route_path and request_path have different length".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello/world".to_string(),
                expected: false,
            },
            TestCase {
                name: "single path parameter".to_string(),
                route_path: "/hello/:name".to_string(),
                request_path: "/hello/world".to_string(),
                expected: true,
            },
            TestCase {
                name: "single query parameter".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello?name=world".to_string(),
                expected: true,
            },
        ];

        for test_case in test_cases.iter() {
            let result = is_route_matching_request(
                test_case.route_path.clone(),
                test_case.request_path.as_str(),
            );

            assert_eq!(
                result, test_case.expected,
                "TC name: {}, route_path: {}, request_path: {}",
                test_case.name, test_case.route_path, test_case.request_path
            );
        }
    }

    #[test]
    fn test_normalize_path() {
        struct TestCase {
            name: String,
            prefix: String,
            path: String,
            expected: String,
        }

        let test_cases = vec![
            TestCase {
                name: "prefix and path are empty".to_string(),
                prefix: "".to_string(),
                path: "".to_string(),
                expected: "/".to_string(),
            },
            TestCase {
                name: "prefix is empty".to_string(),
                prefix: "".to_string(),
                path: "/hello".to_string(),
                expected: "/hello".to_string(),
            },
            TestCase {
                name: "path is empty".to_string(),
                prefix: "/hello".to_string(),
                path: "".to_string(),
                expected: "/hello".to_string(),
            },
            TestCase {
                name: "prefix and path are not empty".to_string(),
                prefix: "/hello".to_string(),
                path: "/world".to_string(),
                expected: "/hello/world".to_string(),
            },
            TestCase {
                name: "prefix and path are not empty and have trailing slashes".to_string(),
                prefix: "/hello/".to_string(),
                path: "/world/".to_string(),
                expected: "/hello/world".to_string(),
            },
        ];

        for test_case in test_cases.iter() {
            let result = normalize_path(test_case.prefix.clone(), test_case.path.clone());

            assert_eq!(
                result, test_case.expected,
                "TC name: {}, prefix: {}, path: {}",
                test_case.name, test_case.prefix, test_case.path
            );
        }
    }

    #[test]
    fn test_find_route() {
        use crate as rupring;

        #[derive(Debug, Clone, Copy)]
        #[rupring::Module(
            controllers=[HomeController{}], 
            modules=[UserModule{}], 
            providers=[], 
            middlewares=[]
        )]
        pub struct RootModule {}
        
        #[derive(Debug, Clone)]
        #[rupring::Controller(prefix=/, routes=[hello, echo])]
        pub struct HomeController {}
        
        #[rupring::Get(path = /)]
        pub fn hello(_request: rupring::Request) -> rupring::Response {
            rupring::Response::new().redirect("https://naver.com")
        }
        
        #[rupring::Get(path = /user)]
        pub fn get_user(_: rupring::Request, _: rupring::Response) -> rupring::Response {
            rupring::Response::new().text("asdf")
        }
        
        #[rupring::Get(path = /echo)]
        pub fn echo(request: rupring::Request, _: rupring::Response) -> rupring::Response {
            rupring::Response::new().text(request.body)
        }
      
        #[derive(Debug, Clone, Copy)]
        #[rupring::Module(
            controllers=[UserController{}],
            modules=[],
            providers=[
            ],
            middlewares=[]
        )]
        pub struct UserModule {}
        
        #[derive(Debug, Clone)]
        #[crate::Controller(prefix=/, routes=[get_user], middlewares=[])]
        pub struct UserController {}

        struct Argument {
            request_path: String,
            request_method: Method,
        }

        struct TestCase {
            name: String,
            argument: Argument,
            expected: Option<String>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                name: "route not found (존재하지 않는 경로)".to_string(),
                argument: Argument {
                    request_path: "/asdf".to_string(),
                    request_method: Method::GET,
                },
                expected: None,
            },
            TestCase {
                name: "route found (/)".to_string(),
                argument: Argument {
                    request_path: "/".to_string(),
                    request_method: Method::GET,
                },
                expected: Some("/".to_string()),
            },
            TestCase {
                name: "route not found (METHOD가 다름)".to_string(),
                argument: Argument {
                    request_path: "/user".to_string(),
                    request_method: Method::POST,
                },
                expected: None,
            },
            TestCase {
                name: "route found (echo)".to_string(),
                argument: Argument {
                    request_path: "/echo".to_string(),
                    request_method: Method::GET,
                },
                expected: Some("/echo".to_string()),
            },
        ];

        for test_case in test_cases.iter() {
            let result = find_route(
                Box::new(RootModule {}),
                test_case.argument.request_path.as_str(),
                &test_case.argument.request_method,
            )
            .map(|(_, route_path, _)| route_path);

            assert_eq!(result, test_case.expected, "TC name: {}", test_case.name,);
        }
    }
}
