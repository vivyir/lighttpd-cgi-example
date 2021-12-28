use qstring::QString;
use std::{collections::HashMap, env};

fn get_query() -> HashMap<String, String> {
    let res = QString::from(
        env::vars()
            .into_iter()
            .collect::<HashMap<String, String>>()
            .get("QUERY_STRING")
            .unwrap()
            .as_str(),
    );

    let mut hm = HashMap::new();

    for (k, v) in res {
        hm.insert(k.to_string(), v.to_string());
    }

    hm
}

fn main() {
    println!("Content-Type: text/plain\n");
    let query = get_query();
    println!("{:#?}", query.into_iter());
}
