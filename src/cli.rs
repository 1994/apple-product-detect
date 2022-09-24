use dialoguer::{Input, MultiSelect};
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use crate::monitor::DetectRequest;
use crate::utils::get_mapper_config;

pub fn get_request<'a>() -> anyhow::Result<(String, DetectRequest)> {

    println!("请选择需要关注的 iPhone型号(按空格选择，回车确认)");

    let items:Vec<&str> =  get_mapper_config().iter()
        .map(|p| {
            p.name.as_str()
        })
        .collect();

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .interact_on_opt(&Term::stderr())?;

    let p: Vec<String> = selection.expect("").into_iter().map(|x| get_mapper_config().get(x).expect(""))
        .map(|p| p.code.to_owned())
        .collect();

    let location : String = Input::new()
        .with_prompt("请输入所在区域(ex: 浙江 杭州 余杭区)")
        .interact_text()?;

    let web_hook : String = Input::new()
        .with_prompt("请输入钉钉机器人hook")
        .interact_text()?;

    Ok((web_hook, DetectRequest{ products: p, location}))
}