use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename="@type")]
    pub typ: String,
    #[serde(rename="@name")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Template {
    data: Vec<Data>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename="@level")]
    pub level: Option<String>,
    #[serde(rename="@symbol")]
    pub symbol: String,
    #[serde(rename="@message")]
    pub message: String,
    #[serde(rename="@value")]
    pub value: u32,
    #[serde(rename="@version")]
    pub version: Option<u8>,
    pub template: Option<Template>,
}

#[derive(Deserialize, Debug)]
pub struct Events {
    pub event: Vec<Event>,
}

#[derive(Deserialize, Debug)]
pub struct Provider {
    pub events: Option<Events>,
}

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub provider: Vec<Provider>,
}

pub fn parse_manifest(fname: String) -> Manifest {
    let content = std::fs::read_to_string(fname).unwrap();
    let doc : Manifest = quick_xml::de::from_str(&*content).unwrap();
    return doc;
}
