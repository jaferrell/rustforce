use crate::utils::{deserialize_bool, deserialize_datetime, deserialize_date};
use chrono::{NaiveDate, DateTime, FixedOffset};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvestmentGroup { 

	#[serde(rename(serialize = "Id", deserialize = "Id"))]
	pub id : Option<String>,

	#[serde(rename(serialize = "OwnerId", deserialize = "OwnerId"))]
	pub owner_id : Option<String>,

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

	#[serde(deserialize_with="deserialize_bool")]
	#[serde(rename(serialize = "Aggregate__c", deserialize = "Aggregate__c"))]
	pub aggregate : Option<bool>,

	#[serde(rename(serialize = "Fund__c", deserialize = "Fund__c"))]
	pub fund : Option<String>,

	#[serde(rename(serialize = "Name__c", deserialize = "Name__c"))]
	pub name__c : Option<String>,

	#[serde(rename(serialize = "Order__c", deserialize = "Order__c"))]
	pub order : Option<f64>,

	#[serde(rename(serialize = "Parent_Investment_Group__c", deserialize = "Parent_Investment_Group__c"))]
	pub parent_investment_group : Option<String>,

	#[serde(deserialize_with="deserialize_bool")]
	#[serde(rename(serialize = "Subtotal__c", deserialize = "Subtotal__c"))]
	pub subtotal : Option<bool>,

	#[serde(rename(serialize = "Type__c", deserialize = "Type__c"))]
	pub r#type : Option<String>,

	#[serde(deserialize_with="deserialize_datetime")]
	#[serde(rename(serialize = "Last_CF_Refresh__c", deserialize = "Last_CF_Refresh__c"))]
	pub last_cf_refresh : Option<DateTime<FixedOffset>>,

	#[serde(deserialize_with="deserialize_datetime")]
	#[serde(rename(serialize = "Last_NAV_Refresh__c", deserialize = "Last_NAV_Refresh__c"))]
	pub last_nav_refresh : Option<DateTime<FixedOffset>>,

	#[serde(rename(serialize = "Memo__c", deserialize = "Memo__c"))]
	pub memo : Option<String>,

	#[serde(deserialize_with="deserialize_date")]
	#[serde(rename(serialize = "CF_Retain__c", deserialize = "CF_Retain__c"))]
	pub cf_retain : Option<NaiveDate>,

}
