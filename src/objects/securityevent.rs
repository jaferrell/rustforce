use crate::utils::{deserialize_bool, deserialize_datetime, deserialize_date};
use chrono::{NaiveDate, DateTime, FixedOffset};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityEvent { 

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

	#[serde(deserialize_with="deserialize_date")]
	#[serde(rename(serialize = "AccrualEndDate__c", deserialize = "AccrualEndDate__c"))]
	pub accrual_end_date : Option<NaiveDate>,

	#[serde(deserialize_with="deserialize_date")]
	#[serde(rename(serialize = "AccrualStartDate__c", deserialize = "AccrualStartDate__c"))]
	pub accrual_start_date : Option<NaiveDate>,

	#[serde(rename(serialize = "Amount__c", deserialize = "Amount__c"))]
	pub amount : Option<f64>,

	#[serde(rename(serialize = "Basis__c", deserialize = "Basis__c"))]
	pub basis : Option<String>,

	#[serde(rename(serialize = "BegOriginalPercent__c", deserialize = "BegOriginalPercent__c"))]
	pub beg_original_percent : Option<f64>,

	#[serde(rename(serialize = "Benchmark__c", deserialize = "Benchmark__c"))]
	pub benchmark : Option<String>,

	#[serde(deserialize_with="deserialize_date")]
	#[serde(rename(serialize = "ComputeDate__c", deserialize = "ComputeDate__c"))]
	pub compute_date : Option<NaiveDate>,

	#[serde(rename(serialize = "DayCount__c", deserialize = "DayCount__c"))]
	pub day_count : Option<String>,

	#[serde(deserialize_with="deserialize_date")]
	#[serde(rename(serialize = "FixDate__c", deserialize = "FixDate__c"))]
	pub fix_date : Option<NaiveDate>,

	#[serde(rename(serialize = "FixRate__c", deserialize = "FixRate__c"))]
	pub fix_rate : Option<f64>,

	#[serde(rename(serialize = "FixedRate__c", deserialize = "FixedRate__c"))]
	pub fixed_rate : Option<f64>,

	#[serde(rename(serialize = "FloatingCeiling__c", deserialize = "FloatingCeiling__c"))]
	pub floating_ceiling : Option<f64>,

	#[serde(rename(serialize = "FloatingFloor__c", deserialize = "FloatingFloor__c"))]
	pub floating_floor : Option<f64>,

	#[serde(deserialize_with="deserialize_bool")]
	#[serde(rename(serialize = "HasCeiling__c", deserialize = "HasCeiling__c"))]
	pub has_ceiling : Option<bool>,

	#[serde(deserialize_with="deserialize_bool")]
	#[serde(rename(serialize = "HasFloor__c", deserialize = "HasFloor__c"))]
	pub has_floor : Option<bool>,

	#[serde(deserialize_with="deserialize_bool")]
	#[serde(rename(serialize = "IsFloating__c", deserialize = "IsFloating__c"))]
	pub is_floating : Option<bool>,

	#[serde(deserialize_with="deserialize_date")]
	#[serde(rename(serialize = "PayDate__c", deserialize = "PayDate__c"))]
	pub pay_date : Option<NaiveDate>,

	#[serde(rename(serialize = "Security__c", deserialize = "Security__c"))]
	pub security : Option<String>,

	#[serde(rename(serialize = "Type__c", deserialize = "Type__c"))]
	pub r#type : Option<String>,

	#[serde(rename(serialize = "Priority__c", deserialize = "Priority__c"))]
	pub priority : Option<f64>,

}
