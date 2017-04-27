//! The utils::authorization module holds logic surrounding validating and processing various
//! OAuth 2.0 Token retrieval requests. In particular, there should be one function designed to handle
//! a particular grant type request. Stylistically these functions are named after the grant type they
//! are processing, and conform to the following function signature, which gives them access to the 
//! underlying datastore as well as the entire request data sent by the caller.

use utils;
use models::requests::*;
use models::responses::*;
use diesel::pg::PgConnection;

/// Processes an `authorization_code` request, and returns a Result on whether or not it was successful.
///
/// Returns: Result<AccessTokenResponse, OAuth2Error>
///          - Ok(AccessTokenResponse) if the request was accepted
///          - Err(OAuth2Error) prefilled with an error message if something went wrong.
pub fn authorization_code(conn: &PgConnection, req: AccessTokenRequest) ->  Result<AccessTokenResponse, OAuth2Error> {
  // Authorization Code requess use the following fields:
  // - (R) grant_type: Should always be "authorization_code", and is expected to have been previously confirmed.
  // - (R) client_id: The client identifier of a previously created Client.
  // - (R) client_secret: The client secret of a previously created Client.
  // - (R) code: The authorization code given to the client after authorization
  // - (O) redirect_uri: The redirect uri sent during the authorization stage, if one was sent.

  if req.client_id.is_none() || req.client_secret.is_none() || req.code.is_none() {
    return Err(utils::oauth_error("invalid_request"));
  }
  let client = match utils::check_client_credentials(conn, &req.client_id.unwrap(), &req.client_secret.unwrap()) {
    Ok(c) => c,
    Err(msg) => return Err(utils::oauth_error(&msg))
  };
  let grant_type = match utils::check_grant_type(conn, &req.grant_type.unwrap()) {
    Ok(g) => g,
    Err(msg) => return Err(utils::oauth_error(&msg))
  };
  
  // As this is stubbed out for now, we return the unsupported grant error message.
  Err(utils::oauth_error("unsupported_grant_type"))
}

/// Processes a `client_credentials` request, and returns a Result on whether or not it was successful.
///
/// Returns: Result<AccessTokenResponse, OAuth2Error>
///          - Ok(AccessTokenResponse) if the request was accepted
///          - Err(OAuth2Error) prefilled with an error message if something went wrong.
pub fn client_credentials(conn: &PgConnection, req: AccessTokenRequest) -> Result<AccessTokenResponse, OAuth2Error> {
  // Client Credentials requests uses the following fields:
  // - (R) client_id: The client identifier of a previously created Client.
  // - (R) client_secret: The client secret of a previously created Client.
  // - (R) scope: The scopes for which this token should be valid.
  if req.client_id.is_none() || req.client_secret.is_none() || req.scope.is_none() {
    return Err(utils::oauth_error("invalid_request"));
  }
  let client = match utils::check_client_credentials(conn, &req.client_id.unwrap(), &req.client_secret.unwrap()) {
    Ok(c) => c,
    Err(msg) => return Err(utils::oauth_error(&msg))
  };
  let grant_type = match utils::check_grant_type(conn, &req.grant_type.unwrap()) {
    Ok(g) => g,
    Err(msg) => return Err(utils::oauth_error(&msg))
  };
  let scope = &req.scope.unwrap();
  let at = utils::generate_access_token(conn, &client, &grant_type, scope);
  let rt = utils::generate_refresh_token(conn, &client, scope);
  Ok(utils::generate_token_response(at, Some(rt)))
}

/// Processes a `refresh_token` request, and returns a Result on whether or not it was successful.
///
/// Returns: Result<AccessTokenResponse, OAuth2Error>
///          - Ok(AccessTokenResponse) if the request was accepted
///          - Err(OAuth2Error) prefilled with an error message if something went wrong.
pub fn refresh_token(conn: &PgConnection, req: AccessTokenRequest) ->  Result<AccessTokenResponse, OAuth2Error> {
  // Refresh Token requests uses the following fields:
  // - (R) grant_type: Should always be "refresh_token", but we expect that to have been previously verified for this request.
  // - (R) refresh_token: The refresh token a client was given when they initially requested an access token.
  // - (O) scope: A scope to request, if you require a REDUCED set of scopes than what was originally used to generate the first token.
  if req.refresh_token.is_none() || req.scope.is_none() {
    return Err(utils::oauth_error("invalid_request"));
  }

  let refresh_token = match utils::check_refresh_token(conn, req.refresh_token.clone().unwrap()) {
    Ok(record) => record,
    Err(_) => return Err(utils::oauth_error("invalid_request"))
  };

  let scope = match utils::check_scope(conn, req.scope.unwrap(), refresh_token.scope.clone()) {
    Ok(s) => s,
    Err(msg) => return Err(utils::oauth_error(&msg))
  };


  // TODO: client should be grabbed from both RefreshToken and request authentication and checked for consistency for security reasons
  let client = utils::get_client_by_id(conn, refresh_token.client_id);
  let grant_type = utils::get_grant_type_by_name(conn, "refresh_token");
  let access_token = utils::generate_access_token(conn, &client, &grant_type, &scope);
  Ok(utils::generate_token_response(access_token, Some(refresh_token)))
}





