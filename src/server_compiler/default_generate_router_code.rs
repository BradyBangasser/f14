use super::server_compilers::Route;

pub fn default_generate_router_code(routes: Vec<Route>) -> Result<Vec<String>, String> {
    let mut router_code: Vec<String> = vec![];
    for route in routes {
        router_code.push(route.to_string())
    };
    
    Ok(router_code)
}