#![allow(non_snake_case)]

extern crate reqwest;
use serde::{Deserialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct QueryResponse<T> {
    pub totalSize: i32,
    pub done: bool,
    pub records: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    pub id: String,
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct UpsertResponse {
    create: Option<CreateResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
    pub error_code: String,
    pub fields: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub id: String,
    pub issued_at: String,
    pub access_token: String,
    pub instance_url: String,
    pub signature: String,
    pub token_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TokenErrorResponse {
    error: String,
    error_description: String,
}

#[derive(Debug)]
pub struct AccessToken {
    pub token_type: String,
    pub value: String,
    pub issued_at: String,
}




#[derive(Deserialize, Debug)]
pub struct SObjectDescribeResponse {

    pub name: String,
    pub fields : Vec<Field>,
    pub recordTypeInfos : Vec<RecordTypeInfo>
}


#[derive(Deserialize, Debug)]
pub struct Field {
    pub name : String,
    pub label : String,
    pub picklistValues : Vec<PicklistValue>,
    pub r#type : String,
    pub defaultedOnCreate : Value,
    pub dependentPicklist : bool,
    pub controllerName : Option<String>,
    pub referenceTo: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PicklistValue {
    pub value : String,
    pub validFor : Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RecordTypeInfo {
    pub name : String,
    pub recordTypeId : String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SObjectRAWDescribeResponse {
    pub fields : serde_json::Value
}
 
// #[derive(Deserialize, Debug)]
// pub struct DescribeGlobalResponse {
//     pub encoding: String,
//     pub maxBatchSize: u16,
//     pub sobjects: Vec<DescribeGlobalSObjectResponse>,
// }


#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub searchRecords: Vec<SearchRecord>,
    //    pub metadata: Metadata,
}

#[derive(Deserialize, Debug)]
pub struct SearchRecord {
    pub id: String,
    pub attributes: SObjectAttribute,
}

#[derive(Deserialize, Debug)]
pub struct SObjectAttribute {
    pub r#type: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionResponse {
    pub label: String,
    pub url: String,
    pub version: String,
}
