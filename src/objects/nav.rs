use crate::utils::{deserialize_bool, deserialize_datetime, deserialize_date};
use chrono::{NaiveDate, DateTime, FixedOffset};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nav { 

	#[serde(rename(serialize = "Id", deserialize = "Id"))]
	pub id : Option<String>,

	#[serde(rename(serialize = "Name", deserialize = "Name"))]
	pub name : Option<String>,

	#[serde(deserialize_with="deserialize_datetime")]
	#[serde(rename(serialize = "CreatedDate", deserialize = "CreatedDate"))]
	pub created_date : Option<DateTime<FixedOffset>>,

	#[serde(rename(serialize = "CreatedById", deserialize = "CreatedById"))]
	pub created_by_id : Option<String>,

	#[serde(deserialize_with="deserialize_datetime")]
	#[serde(rename(serialize = "LastModifiedDate", deserialize = "LastModifiedDate"))]
	pub last_modified_date : Option<DateTime<FixedOffset>>,

	#[serde(rename(serialize = "LastModifiedById", deserialize = "LastModifiedById"))]
	pub last_modified_by_id : Option<String>,

	#[serde(deserialize_with="deserialize_datetime")]
	#[serde(rename(serialize = "SystemModstamp", deserialize = "SystemModstamp"))]
	pub system_modstamp : Option<DateTime<FixedOffset>>,

	#[serde(deserialize_with="deserialize_datetime")]
	#[serde(rename(serialize = "LastViewedDate", deserialize = "LastViewedDate"))]
	pub last_viewed_date : Option<DateTime<FixedOffset>>,

	#[serde(deserialize_with="deserialize_datetime")]
	#[serde(rename(serialize = "LastReferencedDate", deserialize = "LastReferencedDate"))]
	pub last_referenced_date : Option<DateTime<FixedOffset>>,

	#[serde(rename(serialize = "Investment_Group__c", deserialize = "Investment_Group__c"))]
	pub investment_group : Option<String>,

	#[serde(deserialize_with="deserialize_date")]
	#[serde(rename(serialize = "Date__c", deserialize = "Date__c"))]
	pub date : Option<NaiveDate>,

	#[serde(rename(serialize = "Fund__c", deserialize = "Fund__c"))]
	pub fund : Option<String>,

	#[serde(rename(serialize = "Memo__c", deserialize = "Memo__c"))]
	pub memo : Option<String>,

	#[serde(rename(serialize = "Status__c", deserialize = "Status__c"))]
	pub status : Option<String>,

	#[serde(rename(serialize = "Value__c", deserialize = "Value__c"))]
	pub value : Option<f64>,

	#[serde(rename(serialize = "Local_Value__c", deserialize = "Local_Value__c"))]
	pub local_value : Option<f64>,

	#[serde(rename(serialize = "Local_Value_Currency__c", deserialize = "Local_Value_Currency__c"))]
	pub local_value_currency : Option<String>,

	#[serde(rename(serialize = "IRR__c", deserialize = "IRR__c"))]
	pub irr : Option<f64>,

	#[serde(rename(serialize = "Local_IRR__c", deserialize = "Local_IRR__c"))]
	pub local_irr : Option<f64>,

	#[serde(rename(serialize = "Security__c", deserialize = "Security__c"))]
	pub security : Option<String>,

	#[serde(rename(serialize = "AccruedInterest__c", deserialize = "AccruedInterest__c"))]
	pub accrued_interest : Option<f64>,

	#[serde(rename(serialize = "Count__c", deserialize = "Count__c"))]
	pub count : Option<f64>,

	#[serde(rename(serialize = "Price__c", deserialize = "Price__c"))]
	pub price : Option<f64>,

}
