use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(super) struct Parameters {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicState {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) personas: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) devices: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) groups: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) resources: Vec<String>,
}

impl PublicState {
    pub fn field_ref(kind: DidKind) -> fn(&PublicState) -> &Vec<String> {
        match kind {
            DidKind::Persona => |s| &s.personas,
            DidKind::Device => |s| &s.devices,
            DidKind::Group => |s| &s.groups,
            DidKind::Resource => |s| &s.resources,
        }
    }

    pub fn field_mut(kind: DidKind) -> fn(&mut PublicState) -> &mut Vec<String> {
        match kind {
            DidKind::Persona => |s| &mut s.personas,
            DidKind::Device => |s| &mut s.devices,
            DidKind::Group => |s| &mut s.groups,
            DidKind::Resource => |s| &mut s.resources,
        }
    }
}
