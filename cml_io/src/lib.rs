use std::vec::Vec;





pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut res = Vec::new();

    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust: \n productive \n fast \n pick it...";
        
        assert_eq!(search(query, contents).len(), 1);

    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}