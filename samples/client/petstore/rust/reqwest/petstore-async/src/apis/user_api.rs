/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;
use std::borrow::Borrow;
use std::option::Option;

use reqwest;

use super::{Error, configuration};


    pub async fn create_user(configuration: &configuration::Configuration, body: crate::models::User) -> Result<(), Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn create_users_with_array_input(configuration: &configuration::Configuration, body: Vec<crate::models::User>) -> Result<(), Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user/createWithArray", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn create_users_with_list_input(configuration: &configuration::Configuration, body: Vec<crate::models::User>) -> Result<(), Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user/createWithList", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn delete_user(configuration: &configuration::Configuration, username: &str) -> Result<(), Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user/{username}", configuration.base_path, username=crate::apis::urlencode(username));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn get_user_by_name(configuration: &configuration::Configuration, username: &str) -> Result<crate::models::User, Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user/{username}", configuration.base_path, username=crate::apis::urlencode(username));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<crate::models::User>().await?)
    }

    pub async fn login_user(configuration: &configuration::Configuration, username: &str, password: &str) -> Result<String, Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user/login", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("username", &username.to_string())]);
        req_builder = req_builder.query(&[("password", &password.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<String>().await?)
    }

    pub async fn logout_user(configuration: &configuration::Configuration, ) -> Result<(), Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user/logout", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn update_user(configuration: &configuration::Configuration, username: &str, body: crate::models::User) -> Result<(), Error> {
        let client = &configuration.client;

        let uri_str = format!("{}/user/{username}", configuration.base_path, username=crate::apis::urlencode(username));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

