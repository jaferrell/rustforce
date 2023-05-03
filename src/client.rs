#[allow(non_snake_case)]

extern crate reqwest;
use crate::errors::Error;
use crate::response::{
    AccessToken, CreateResponse, SObjectDescribeResponse, SObjectRAWDescribeResponse, ErrorResponse,
    QueryResponse, SearchResponse, TokenResponse, VersionResponse,
};

use crate::utils::substring_before;
use regex::Regex;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use serde::de;

/// Represents a Salesforce Client
pub struct Client {
    http_client: reqwest::Client,
    client_id: Option<String>,
    client_secret: Option<String>,
    login_endpoint: String,
    instance_url: Option<String>,
    access_token: Option<AccessToken>,
    version: String,
}

impl Client {
    
    pub fn new() -> Self {
        let http_client = reqwest::Client::new();
        Client {
            http_client,
            client_id : None,
            client_secret : None,
            login_endpoint: "https://login.salesforce.com".to_string(),
            access_token: None,
            instance_url: None,
            version: "v57.0".to_string(),
        }
    }

    /// Set the login endpoint. This is useful if you want to connect to a test or developer
    pub fn set_login_endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.login_endpoint = endpoint.to_string();
        self
    }

    pub fn set_client_id(&mut self, client_id : &str) -> &mut Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    pub fn set_client_secret(&mut self, client_secret : &str) -> &mut Self {
        self.client_secret = Some(client_secret.to_string());
        self
    }

    /// Set API Version
    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version = version.to_string();
        self
    }

    pub fn set_instance_url(&mut self, instance_url: &str) -> &mut Self {
        self.instance_url = Some(instance_url.to_string());
        self
    }

    /// Set Access token if you've already obtained one via one of the OAuth2 flows
    fn set_access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = Some(AccessToken {
            token_type: "Bearer".to_string(),
            value: access_token.to_string(),
            issued_at: "".to_string(),
        });
        self
    }

    /// This will fetch an access token when provided with a refresh token
    pub async fn refresh(&mut self, refresh_token: &str) -> Result<&mut Self, Error> {
        let token_url = format!("{}/services/oauth2/token", self.login_endpoint);
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", self.client_id.as_ref().unwrap()),
            ("client_secret", self.client_secret.as_ref().unwrap()),
        ];
        let res = self
            .http_client
            .post(token_url.as_str())
            .form(&params)
            .send()
            .await?;

        if res.status().is_success() {
            let r: TokenResponse = res.json().await?;
            self.access_token = Some(AccessToken {
                value: r.access_token,
                issued_at: r.issued_at,
                token_type: "Bearer".to_string(),
            });
            self.instance_url = Some(r.instance_url);
            Ok(self)
        } else {
            let token_error = res.json().await?;
            Err(Error::TokenError(token_error))
        }
    }

    /// Login to Salesforce with username and password
    pub async fn login_with_credential(
        &mut self,
        username: String,
        password: String,
    ) -> Result<&mut Self, Error> {
        let token_url = format!("{}/services/oauth2/token", self.login_endpoint);
        let params = [
            ("grant_type", "password"),
            ("client_id", self.client_id.as_ref().unwrap()),
            ("client_secret", self.client_secret.as_ref().unwrap()),
            ("username", username.as_str()),
            ("password", password.as_str()),
        ];
        let res = self
            .http_client
            .post(token_url.as_str())
            .form(&params)
            .send()
            .await?;

        if res.status().is_success() {
            let r: TokenResponse = res.json().await?;
            self.access_token = Some(AccessToken {
                value: r.access_token,
                issued_at: r.issued_at,
                token_type: r.token_type.ok_or(Error::NotLoggedIn)?,
            });
            self.instance_url = Some(r.instance_url);
            Ok(self)
        } else {
            let error_response = res.json().await?;
            Err(Error::TokenError(error_response))
        }
    }

    pub async fn login_by_soap(&mut self, username: String, password: String) -> Result<&mut Self, Error> {
        let token_url = format!(
            "{login_endpoint}/services/Soap/u/{version}",
            login_endpoint = self.login_endpoint,
            version = self.version
        );
        let body = [
            "<se:Envelope xmlns:se='http://schemas.xmlsoap.org/soap/envelope/'>",
            "<se:Header/>",
            "<se:Body>",
            "<login xmlns='urn:partner.soap.sforce.com'>",
            format!("<username>{}</username>", username).as_str(),
            format!("<password>{}</password>", password).as_str(),
            "</login>",
            "</se:Body>",
            "</se:Envelope>",
        ]
        .join("");
        let res = self
            .http_client
            .post(token_url.as_str())
            .body(body)
            .header("Content-Type", "text/xml")
            .header("SOAPAction", "\"\"")
            .send()
            .await?;
        if res.status().is_success() {
            let body_response = res.text().await?;
            let re_access_token = Regex::new(r"<sessionId>([^<]+)</sessionId>").unwrap();
            let re_instance_url = Regex::new(r"<serverUrl>([^<]+)</serverUrl>").unwrap();
            self.access_token = Some(AccessToken {
                value: String::from(
                    re_access_token
                        .captures(body_response.as_str())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                ),
                issued_at: "".to_string(),
                token_type: "Bearer".to_string(),
            });
            self.instance_url = Some(substring_before(
                re_instance_url
                    .captures(body_response.as_str())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str(),
                "/services/",
            ));
            Ok(self)
        } else {
            let body_response = res.text().await?;
            let re_message = Regex::new(r"<faultstring>([^<]+)</faultstring>").unwrap();
            let re_error_code = Regex::new(r"<faultcode>([^<]+)</faultcode>").unwrap();
            Err(Error::LoginError(ErrorResponse {
                message: String::from(
                    re_message
                        .captures(body_response.as_str())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                ),
                error_code: String::from(
                    re_error_code
                        .captures(body_response.as_str())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                ),
                fields: None,
            }))
        }
    }

    // ADVANCED QUERY WITH ALL FIELDS
    pub async fn query_all_all_fields<T : de::DeserializeOwned>(&self, sobject_name : &str) -> Result<Vec<T >, Error> {
        let soql = self.create_all_fields_SOQL(&sobject_name).await?;
        let res  = self.query_all::<T>(&soql).await?;
        Ok(res.records)
    }

    // GET ALL FIELDS
    pub async fn get_all_fields(&self, sobject_name: &str) -> Result<Vec<String>, Error> {
        let sobject = self.describe(&sobject_name).await?;
        let mut fields = Vec::<String>::new();

        for field in sobject.fields.iter() {
            fields.push(field.name.clone());
        }
        Ok(fields)
    }

    // CREATE ALL FIELDS SOQL
    pub async fn create_all_fields_SOQL(&self, sobject_name : &str) -> Result<String, Error> {
        
        let fields = self.get_all_fields(sobject_name).await?;
        let mut fields_string = String::new();

        for field in fields.iter() {
            fields_string.push_str(&field);
            fields_string.push_str(", ");
        }
        fields_string.pop();
        fields_string.pop();

        let SOQL = format!("Select {} from {}", fields_string, sobject_name);
        Ok(SOQL)
    }

    /// Query record using SOQL
    pub async fn query<T: DeserializeOwned>(&self, query: &str) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/query/", self.base_path());
        let params = vec![("q", query)];
        let res = self.get(query_url, params).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Query All records using SOQL
    pub async fn query_all<T: DeserializeOwned>(
        &self,
        query: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/queryAll/", self.base_path());
        let params = vec![("q", query)];
        let res = self.get(query_url, params).await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Find records using SOSL
    pub async fn search(&self, query: &str) -> Result<SearchResponse, Error> {
        let query_url = format!("{}/search/", self.base_path());
        let params = vec![("q", query)];
        let res = self.get(query_url, params).await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Get all supported API versions
    pub async fn versions(&self) -> Result<Vec<VersionResponse>, Error> {
        let versions_url = format!(
            "{}/services/data/",
            self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?
        );
        let res = self.get(versions_url, vec![]).await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Finds a record by ID
    pub async fn find_by_id<T: DeserializeOwned>(
        &self,
        sobject_name: &str,
        id: &str,
    ) -> Result<T, Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        let res = self.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Creates an SObject
    pub async fn create<T: Serialize>(
        &self,
        sobject_name: &str,
        params: T,
    ) -> Result<CreateResponse, Error> {
        let resource_url = format!("{}/sobjects/{}", self.base_path(), sobject_name);
        let res = self.post(resource_url, params).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Updates an SObject
    pub async fn update<T: Serialize>(
        &self,
        sobject_name: &str,
        id: &str,
        params: T,
    ) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        let res = self.patch(resource_url, params).await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Upserts an SObject with key
    pub async fn upsert<T: Serialize>(
        &self,
        sobject_name: &str,
        key_name: &str,
        key: &str,
        params: T,
    ) -> Result<Option<CreateResponse>, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}/{}",
            self.base_path(),
            sobject_name,
            key_name,
            key
        );
        let res = self.patch(resource_url, params).await?;

        if res.status().is_success() {
            match res.status() {
                StatusCode::CREATED => Ok(res.json().await?),
                _ => Ok(None),
            }
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Deletes an SObject
    pub async fn destroy(&self, sobject_name: &str, id: &str) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        let res = self.delete(resource_url).await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Describes specific object
    pub async fn describe(&self, sobject_name: &str) -> Result<SObjectDescribeResponse, Error> {
        let resource_url = format!("{}/sobjects/{}/describe", self.base_path(), sobject_name);
        let res = self.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(serde_json::from_str(res.text().await?.as_str())?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn describe_raw(&self, sobject_name: &str) -> Result<SObjectRAWDescribeResponse , Error> {
        let resource_url = format!("{}/sobjects/{}/describe", self.base_path(), sobject_name);
        let res = self.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(serde_json::from_str(res.text().await?.as_str())?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn rest_get(
        &self,
        path: String,
        params: Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .get(url.as_str())
            .headers(self.create_header()?)
            .query(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_post<T: Serialize>(
        &self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .post(url.as_str())
            .headers(self.create_header()?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_patch<T: Serialize>(
        &self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .patch(url.as_str())
            .headers(self.create_header()?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_put<T: Serialize>(&self, path: String, params: T) -> Result<Response, Error> {
        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .put(url.as_str())
            .headers(self.create_header()?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_delete(&self, path: String) -> Result<Response, Error> {
        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .delete(url.as_str())
            .headers(self.create_header()?)
            .send()
            .await?;
        Ok(res)
    }

    async fn get(&self, url: String, params: Vec<(&str, &str)>) -> Result<Response, Error> {
        let res = self
            .http_client
            .get(url.as_str())
            .headers(self.create_header()?)
            .query(&params)
            .send()
            .await?;
        Ok(res)
    }

    async fn post<T: Serialize>(&self, url: String, params: T) -> Result<Response, Error> {
        let res = self
            .http_client
            .post(url.as_str())
            .headers(self.create_header()?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    async fn patch<T: Serialize>(&self, url: String, params: T) -> Result<Response, Error> {
        let res = self
            .http_client
            .patch(url.as_str())
            .headers(self.create_header()?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    async fn delete(&self, url: String) -> Result<Response, Error> {
        let res = self
            .http_client
            .delete(url.as_str())
            .headers(self.create_header()?)
            .send()
            .await?;
        Ok(res)
    }

    fn create_header(&self) -> Result<HeaderMap, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!(
                "Bearer {}",
                self.access_token.as_ref().ok_or(Error::NotLoggedIn)?.value
            )
            .parse()?,
        );

        Ok(headers)
    }

    pub fn base_path(&self) -> String {
        format!(
            "{}/services/data/{}",
            self.instance_url.as_ref().unwrap(),
            self.version
        )
    }
}

