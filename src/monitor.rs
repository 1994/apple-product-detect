use std::collections::HashMap;
use reqwest::{Url};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use crate::utils::{get_by_code, get_client};

const URL: &str = "https://www.apple.com.cn/shop/fulfillment-messages";

#[derive(Debug)]
pub struct DetectResponse {
    available: bool,
    value: Vec<Value>,
}

impl DetectResponse {
    pub fn is_available(&self) -> bool {
        self.available && !self.value.is_empty()
    }
}

impl From<DetectResponse> for String {
    fn from(response: DetectResponse) -> Self {
        let a = response.is_available();
        if !a {
            String::from("no available")
        } else {
            let t: Vec<String> = response.value.iter().map(
                |v| {
                    format!("{}\n{}\n", v.store_name, v.products.join(","))
                }
            ).collect();
            t.join("\n")
        }
    }
}

#[derive(Debug)]
pub struct Value {
    store_name: String,
    products: Vec<String>,
}

#[derive(Debug)]
pub struct DetectRequest {
    pub products: Vec<String>,
    pub location: String,
}

impl From<OriginResponse> for DetectResponse {
    fn from(origin: OriginResponse) -> Self {
        let success = origin.head.status.eq("200");
        if !success {
            return DetectResponse {
                available: false,
                value: vec![],
            };
        }
        let v: Vec<Value> = origin.body.content.pickup_message.stores.into_iter()
            .filter(
                |p| !p.get_available().is_empty()
            ).map(
            |p| {
                let v = p.get_available().into_iter().map(String::from)
                    .map(|s| get_by_code(s).name)
                    .collect();
                Value { store_name: format!("({})[{}]", p.store_name, p.address.address), products: v }
            }
        ).collect();

        DetectResponse { available: true, value: v }
    }
}

pub fn detect(request: &DetectRequest) -> Result<DetectResponse> {
    let l = &request.location;
    // MQ1C3CH/A
    let mut v = vec![
        ("mts.0".to_string(), "regular".to_string()), ("pl".to_string(), "true".to_string()),
        // ("parts.0", "MQD83CH/A"),
        // ("parts.1", "MPXR3CH/A"),
        ("location".to_string(), l.to_string()),
    ];
    let mut parts: Vec<(String, String)> = request.products.iter().enumerate().map(|product| {
        let left = format!("parts.{}", product.0);
        (left, product.1.to_string())
    }).collect();

    v.append(&mut parts);

    let url: Url = Url::parse_with_params(URL, &v)?;
    println!("{:?}", url.to_string());
    let resp = get_client()
        .get(url)
        .send()?;

    let success = resp.status().is_success();
    if !success {
        Ok(DetectResponse{ available: false, value: vec![] })
    }

    let p = resp.json::<OriginResponse>()?;
    Ok(DetectResponse::from(p))
}

#[derive(Deserialize, Debug)]
struct OriginResponse {
    head: Head,
    body: Body,
}

#[derive(Deserialize, Debug)]
struct Head {
    status: String,
}

#[derive(Debug, Deserialize)]
struct Body {
    content: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Content {
    pickup_message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    stores: Vec<Store>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Store {
    store_name: String,
    address: Address,
    parts_availability: HashMap<String, Part>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Address {
    address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Part {
    pickup_display: String,
}

impl Store {
    fn get_available(&self) -> Vec<&String> {
        let result: Vec<&String> = self.parts_availability.iter()
            .filter(|p| p.1.is_available())
            .map(|p|
                p.0
            ).collect();
        result
    }
}

impl Part {
    fn is_available(&self) -> bool {
        self.pickup_display.eq("available")
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use crate::monitor::{detect, DetectRequest};

    #[test]
    fn work() {
        let r = DetectRequest { products: vec!["MQD83CH/A".to_string()], location: "浙江 杭州 余杭区".to_string() };
        let result = detect(&r);
        assert!(result.is_ok())
    }

    #[test]
    fn test2() {
        let a = Arc::new("d".to_string());
        let str = a.to_string();
        println!("{}", str);
    }
}