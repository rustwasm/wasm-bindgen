use std::collections::HashMap;

// Public API types for a TypeScript project based on
// https://github.com/Microsoft/web-build-tools/blob/master/apps/api-extractor/src/api/api-json.schema.json
//
// There are some attributes that are omitted because they are not relevant to
// us.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TsPackage {
    kind: String,
    name: String,
    pub(crate) exports: HashMap<String, TsExport>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub(crate) enum TsExport {
    #[serde(rename = "class")]
    TsClass {
        members: HashMap<String, TsClassMembers>,
    },

    #[serde(rename = "function")]
    TsFunction {
        parameters: HashMap<String, TsMethodProperty>,
        #[serde(rename = "returnValue")]
        return_value: TsReturnValue,
    },
    //TODO: implement ...
    //{ "$ref": "#/definitions/interfaceApiItem" },
    //{ "$ref": "#/definitions/namespaceApiItem" },
    //{ "$ref": "#/definitions/enumApiItem" },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub(crate) enum TsClassMembers {
    #[serde(rename = "property")]
    TsProperty {
        #[serde(rename = "isStatic")]
        is_static: bool,
        #[serde(rename = "isReadOnly")]
        is_read_only: bool,
        #[serde(rename = "type")]
        property_type: String,
    },

    #[serde(rename = "constructor")]
    TsConstructor {
        parameters: HashMap<String, TsMethodProperty>,
    },

    #[serde(rename = "method")]
    TsMethod {
        #[serde(rename = "accessModifier")]
        access_modifier: String,
        #[serde(rename = "isStatic")]
        is_static: bool,
        parameters: HashMap<String, TsMethodProperty>,
        #[serde(rename = "returnValue")]
        return_value: TsReturnValue,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TsMethodProperty {
    name: String,
    #[serde(rename = "type")]
    pub(crate) property_type: String,
    #[serde(rename = "isSpread")]
    is_spread: bool,
    #[serde(rename = "isOptional")]
    is_optional: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TsReturnValue {
    #[serde(rename = "type")]
    pub(crate) property_type: String,
}
