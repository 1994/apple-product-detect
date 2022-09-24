use lazy_static::lazy_static;
use reqwest::blocking::Client;
use crate::{mapper::get_mapper, mapper::PhoneMapper};

lazy_static! {
    static ref CLIENT : Client = reqwest::blocking::ClientBuilder::new()
    .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0")
    .build()
    .expect("create http client error");

    static ref PHONES: Vec<PhoneMapper> = get_mapper();
}

pub fn get_client() -> &'static Client {
    &CLIENT
}

pub fn get_mapper_config() -> &'static Vec<PhoneMapper> {
    &PHONES
}

pub fn get_by_code(str: String) -> PhoneMapper {
    let p = get_mapper_config().iter().find(|c| c.code.eq(str.as_str())).expect("can't find phone mapper");
    PhoneMapper{ code: p.code.to_string(), name: p.name.to_string() }
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_client, get_mapper_config};

    #[test]
    fn test() {
        let _ = get_client();
    }

    #[test]
    fn test_phone() {
        let p = get_mapper_config();
        assert!(!p.is_empty());
    }
}