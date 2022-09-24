use std::collections::HashMap;
use anyhow::anyhow;
use serde::Serialize;
use crate::utils::get_client;

enum PushType {
    DINGTALK
}

pub struct PushFactory {
    push_type: PushType,
}

impl Default for PushFactory {
    fn default() -> Self {
        PushFactory { push_type: PushType::DINGTALK }
    }
}

impl PushFactory {
    pub(crate) fn push(&self, content: String, web_hook: String) -> anyhow::Result<bool> {
        match self.push_type {
            PushType::DINGTALK => {
                let d = DingTalk { content: content.as_str(), web_hook: web_hook.as_str() };
                d.push()
            }
        }
    }
}

pub trait PushService {
    fn push(&self) -> anyhow::Result<bool>;
}

struct DingTalk<'a> {
    web_hook: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct ContentWrapper {
    content: String,
}

impl<'a> DingTalk<'a> {
    fn get_params(&self) -> HashMap<&str, String> {
        let mut p: HashMap<&str, String> = HashMap::new();
        p.insert("msgtype", "text".to_string());
        let c = ContentWrapper { content: format!("抢抢抢，当前有货\n============={}\n", self.content)};
        let json_str = serde_json::to_string(&c).expect("serialize error");
        p.insert("text", json_str);
        p
    }
}


// curl 'https://oapi.dingtalk.com/robot/send?access_token=xxxxxxxx' \
// -H 'Content-Type: application/json' \
// -d '{"msgtype": "text","text": {"content":"我就是我, 是不一样的烟火"}}'
impl<'a> PushService for DingTalk<'a> {
    fn push(&self) -> anyhow::Result<bool> {
        let request = get_client().post(self.web_hook.to_string())
            .header("Content-Type", "application/json");
        if self.content.is_empty() {
            return Err(anyhow!("DingTalk send error, content illegal"));
        }
        let p = self.get_params();
        let response =
            request.json(&p)
                .send()?;
        println!("{:?}", response.text());
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::push::{DingTalk, PushService};

    #[test]
    fn push_works() {
        let talk = DingTalk { web_hook: "", content: "抢抢抢" };
        let result = talk.push();
        println!("{:?}", result);
        // assert!(talk.push().is_err())
    }
}
