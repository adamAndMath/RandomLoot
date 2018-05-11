use super::item::Item;

pub struct Format();

impl Format {
    pub fn from(s: String) -> Result<Format, String> {
        let vars = s[1..s.len()-2].split("]:[");
        for v in vars {
            println!("{}", v);
        }

        Err("Not implimented")
    }
}