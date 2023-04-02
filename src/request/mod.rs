pub fn get<T>(url: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let res = reqwest::blocking::get(url).expect("Failed to get");
    if !res.status().is_success() {
        println!("Get Http Error: {}", res.status())
    }

    res.json().expect("Failed to parse json")
}
