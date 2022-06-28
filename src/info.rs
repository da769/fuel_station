use std::collections::HashMap;
pub fn prizetag_items() -> HashMap<String, f32>{
    HashMap::from([
        (stg("diesel"),2.09),
        (stg("petrol"),3.10),
        (stg("kerosine"),1.06),
        (stg("gas"),2.80),
    ])
}
fn stg(fuel: &str) -> String{
    String::from(fuel)
}
