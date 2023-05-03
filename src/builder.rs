use std::fs::File;
use std::io::Write;

use std::env;

use crate::client::Client;
use crate::Error;

use convert_case::{Case, Casing};


impl Client {

    pub async fn build_object(&mut self, sobject_name : &str) -> Result<(), Error> {
    
        // let mut client = Client::new();
    
        // dotenv::dotenv().ok();
        // let username = env::var("SALESFORCE_USERNAME").unwrap();
        // let password_token = env::var("SALESFORCE_PASSWORD_TOKEN").unwrap();
    
        // client.login_by_soap(username, password_token).await?;
        
        let sobject = self.describe(&sobject_name).await?;
        
        let file_name = format!("{}.rs", (self.translate_object_name(&sobject_name)).to_ascii_lowercase());
        let mut file = File::create(file_name).expect("Could not create file");
        
        file.write_all(format!("use crate::utils::{{deserialize_bool, deserialize_datetime, deserialize_date}};\n").as_bytes()).unwrap();
        file.write_all(format!("use chrono::{{NaiveDate, DateTime, FixedOffset}};\n\n").as_bytes()).unwrap();
        file.write_all(format!("use serde::{{Serialize, Deserialize}};\n\n").as_bytes()).unwrap();
        file.write_all("#[derive(Serialize, Deserialize, Debug, Clone)]\n".as_bytes()).unwrap();
        file.write_all(format!("pub struct {} {{ \n\n", self.translate_object_name(&sobject_name)).as_bytes()).unwrap();
        for field in sobject.fields.iter() {       
    
            if field.name == "IsDeleted" { continue; }
            
            match field.r#type.as_str()  {
                "boolean" => {   
                    file.write_all(format!("\t#[serde(deserialize_with=\"deserialize_bool\")]\n").as_bytes()).unwrap();
                },
                "double" => {
    
                },
                "date" => {
                    file.write_all(format!("\t#[serde(deserialize_with=\"deserialize_date\")]\n").as_bytes()).unwrap();  
                },
    
                "datetime" => {
                    file.write_all(format!("\t#[serde(deserialize_with=\"deserialize_datetime\")]\n").as_bytes()).unwrap();
                },
    
                _ => {
                },
            }
    
            file.write_all(format!("\t#[serde(rename(serialize = \"{}\", deserialize = \"{}\"))]\n", field.name, field.name).as_bytes()).unwrap(); 
    
            match field.name.as_str() {
                "Type" | "Type__c" => {
                    file.write_all(format!("\tpub {} : {},\n\n",  "r#type", self.translate_field(field.r#type.as_str())).as_bytes()).unwrap();
                },
    
                _ => {
                    file.write_all(format!("\tpub {} : {},\n\n",  self.translate_field_name(&field.name), self.translate_field(field.r#type.as_str())).as_bytes()).unwrap();
                }
            }
            
        }
    
        file.write_all(format!("}}\n").as_bytes()).unwrap();
    
        Ok(())
    
    
    }
    
    fn translate_field_name(&mut self, sobject_field_name : &str) -> String {
        let new_field_name = sobject_field_name.trim_end_matches("__c").to_string();
        new_field_name.to_case(Case::Snake)
    }
    
    fn translate_object_name<'a>(&mut self, sobject_name : &'a str) -> &'a str {
        match sobject_name {
            "Investment_Group__c" => "InvestmentGroup",
            "CashFlow__c" => "Cashflow",
            "NAV__c" => "Nav",
            "Account" => "Account",
            "Security__c" => "Security",
            "SecurityEvent__c" => "SecurityEvent",
            "Security_Price__c" => "SecurityPrice",
            "Yield_Curve__c" => "YieldCurve",
            "Transaction__c" => "Transaction",
            "Rate_Fix__c" => "FixRate",
            "Position_Link__c" => "PositionLink",
            "Position_Group__c" => "PositionGroup",
            "Prospect__c" => "Prospect",
            "Investment_Classifier__c" => "InvestmentClassifier",
            "FX_Table__c" => "FXTable",
            "Deal__c" => "Deal",
            "Contact" => "Contact",
            "SurveyAnswer__c" => "SurveyAnswer",
            "SurveyCell__c" => "SurveyCell",
            "SurveyQuestion__c" => "SurveyQuestion",
            "SurveyAnswerGroup__c" => "SurveyAnswerGroup",
            "SurveyQuestionGroup__c" => "SurveyQuestionGroup",
            "SurveyQuestionFork__c" => "SurveyQuestionFork",
            "Survey__c" => "Survey",
            "Term_Sheet__c" => "TermSheet",
            "User" => "User",
            "Restriction__c" => "Restriction",
            "ContactDealLink__c" => "ContactDealLink",
            "Collateral__c" => "Collateral",
            "Call_Note__c" => "CallNote",
            _ => sobject_name
        }
    }

    fn translate_field(&mut self, field_type : &str) -> &str {
    
        match field_type {
            "id" => "Option<String>",
            "boolean" => "Option<bool>",
            "reference" => "Option<String>",
            "string" => "Option<String>",
            "picklist" => "Option<String>",
            "double" => "Option<f64>",
            "textarea" => "Option<String>",
            "address" => "Option<String>",
            "phone" => "Option<String>",
            "datetime" => "Option<DateTime<FixedOffset>>",
            "url" => "Option<String>",
            "int" => "Option<i32>",
            "date" => "Option<NaiveDate>",
            "multipicklist" => "Option<String>",
            "email" => "Option<String>",
            "currency" => "Option<f64>",
            "percent" => "Option<f64>",
            _ => "None"
        }
    }

}


