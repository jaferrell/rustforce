#[allow(non_snake_case)]

use std::env;



use rustforce::{Client, Error};
use tokio;




use serde::{Serialize,Deserialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Account {
//     pub Name : String,
// }


use rustforce::utils::write_to_file;

use rustforce::objects::*;


#[tokio::main] 
async fn main() {



    // build_object("Investment_Group__c").await.unwrap();


    dotenv::dotenv().ok();
    let username = env::var("SALESFORCE_USERNAME").unwrap();
    let password_token = env::var("SALESFORCE_PASSWORD_TOKEN").unwrap();

    let mut client = Client::new();
    client.login_by_soap(username, password_token).await;
    // client.build_object("SecurityEvent__c").await;
    let accs = client.query_all_all_fields::<SecurityEvent>("SecurityEvent__c").await.unwrap();
    println!("{:?}", accs);
    for acc in accs {
        println!("{:?}", acc);
    }
    
    
    
    
    // let a = client.describe_raw("NAV__c").await.unwrap();
    // write_to_file("nav", a);
    // client.build_object("Investment_Group__c").await;
    // let a = client.describe_raw("Account").await;
    // write_to_file("contents.txt".to_string(), a.unwrap());
    // client.describe_raw("Investment_Group__c");
    
    // println!("{:#?}", igs);
    
    // let mut client = Client::new();
    // client.login_by_soap(username, password_token).await;
 
    
    // client.get_all_fields("Security__c").await;
    // client.create_all_fields_SOQL("Security__c").await;

    // let res = client.rest_get(format!("/services/data/v57.0/ui-api/object-info/Security__c/picklist-values/012000000000000AAA"),vec![]).await;
    // // let res = client.describe("Security__c").await.unwrap();
    
    // println!("{:#?}", res.unwrap().text().await.unwrap());
    
    
    
    
}