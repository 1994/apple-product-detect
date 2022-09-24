const PHONE_MAPPER:&str = r#"
iPhone 14 Pro 128GB 深空黑色 MPXR3CH/A
iPhone 14 Pro 128GB 银色 MPXY3CH/A
iPhone 14 Pro 128GB 金色 MQ053CH/A
iPhone 14 Pro 128GB 暗紫色 MQ0D3CH/A
iPhone 14 Pro 256GB 深空黑色 MQ0M3CH/A
iPhone 14 Pro 256GB 银色 MQ0W3CH/A
iPhone 14 Pro 256GB 金色 MQ143CH/A
iPhone 14 Pro 256GB 暗紫色 MQ1C3CH/A
iPhone 14 Pro 512GB 深空黑色 MQ1J3CH/A
iPhone 14 Pro 512GB 银色 MQ1R3CH/A
iPhone 14 Pro 512GB 金色 MQ203CH/A
iPhone 14 Pro 512GB 暗紫色 MQ263CH/A
iPhone 14 Pro 1TB 深空黑色 MQ2D3CH/A
iPhone 14 Pro 1TB 银色 MQ2K3CH/A
iPhone 14 Pro 1TB 金色 MQ2R3CH/A
iPhone 14 Pro 1TB 暗紫色 MQ2Y3CH/A
iPhone 14 Pro Max 128GB 深空黑色 MQ833CH/A
iPhone 14 Pro Max 128GB 银色 MQ843CH/A
iPhone 14 Pro Max 128GB 金色 MQ853CH/A
iPhone 14 Pro Max 128GB 暗紫色 MQ863CH/A
iPhone 14 Pro Max 256GB 深空黑色 MQ873CH/A
iPhone 14 Pro Max 256GB 银色 MQ883CH/A
iPhone 14 Pro Max 256GB 金色 MQ893CH/A
iPhone 14 Pro Max 256GB 暗紫色 MQ8A3CH/A
iPhone 14 Pro Max 512GB 深空黑色 MQ8D3CH/A
iPhone 14 Pro Max 512GB 银色 MQ8E3CH/A
iPhone 14 Pro Max 512GB 金色 MQ8F3CH/A
iPhone 14 Pro Max 512GB 暗紫色 MQ8G3CH/A
iPhone 14 Pro Max 1TB 深空黑色 MQ8H3CH/A
iPhone 14 Pro Max 1TB 银色 MQ8J3CH/A
iPhone 14 Pro Max 1TB 金色 MQ8L3CH/A
iPhone 14 Pro Max 1TB 暗紫色 MQ8M3CH/A
"#;

#[derive(Debug)]
pub struct PhoneMapper {
    pub code :String,
    pub name: String
}

pub fn get_mapper() -> Vec<PhoneMapper> {
    let mut v:Vec<PhoneMapper> = PHONE_MAPPER.split('\n').into_iter().filter(| s | !s.is_empty() && s.contains("MQ"))
    .map(|s| {
        let all : Vec<&str> = s.split("MQ").collect();
        let first = all.first().expect("parser phone config error");
        let second = all.get(1).expect("parser phone config error");
        PhoneMapper{ code: format!("MQ{}", second).trim().to_string(), name: first.to_string().trim().to_string() }
    }).collect();
    v.insert(0, PhoneMapper{ code: "MPXR3CH/A".to_string(), name: "iPhone 14 Pro 128GB 深空黑色".to_string() });
    v.insert(1, PhoneMapper{ code: "MPXY3CH/A".to_string(), name: "iPhone 14 Pro 128GB 银色".to_string() });
    v

}