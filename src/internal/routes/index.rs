
pub fn root(request: &mut nickel::Request) -> String {
    //TODO :  Display mapping of the index and some informations (eg: size, name, ...)
    format!("ROOT {}", request.param("name").unwrap().to_string())
}

pub fn count(request: &mut nickel::Request) -> String {
    //TODO : Count the number of document inside an index
    format!("COUNT {}", request.param("name").unwrap().to_string())
}

pub fn search(request: &mut nickel::Request) -> String {
    //TODO : Search document in index
    format!("SEARCH {}", request.param("name").unwrap().to_string())
}