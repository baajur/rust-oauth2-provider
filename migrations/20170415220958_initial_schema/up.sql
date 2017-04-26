-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE clients (
  id SERIAL PRIMARY KEY,
  identifier VARCHAR(256) NOT NULL,
  secret VARCHAR(256) NOT NULL,
  response_type VARCHAR(64) NOT NULL,
  CONSTRAINT clients__unique_identifier
    UNIQUE (identifier)
);

CREATE TABLE grant_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) NOT NULL,
  CONSTRAINT grant_types__unique_name
    UNIQUE (name)
);

CREATE TABLE client_redirect_uris (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL,
  redirect_uri VARCHAR(128) NOT NULL,
  CONSTRAINT client_redirect_uris__client_id
    FOREIGN KEY (client_id)
    REFERENCES clients (id)
);

CREATE TABLE access_tokens (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL,
  grant_id INTEGER NOT NULL,
  token uuid NOT NULL DEFAULT uuid_generate_v4(),
  refresh_token uuid NOT NULL DEFAULT uuid_generate_v4(),
  scope VARCHAR(255) NOT NULL,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  issued_at TIMESTAMP WITH TIME ZONE NOT NULL,
  refresh_expires_at TIMESTAMP WITH TIME ZONE,
  CONSTRAINT access_tokens__client_id
    FOREIGN KEY (client_id)
    REFERENCES clients (id),
  CONSTRAINT access_tokens__grant_id
    FOREIGN KEY (grant_id)
    REFERENCES grant_types (id),
  CONSTRAINT access_tokens__unique_token
    UNIQUE(token),
  CONSTRAINT access_tokens__unique_refresh_token
    UNIQUE(refresh_token)
);

CREATE TABLE auth_codes (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL,
  name VARCHAR(64) NOT NULL,
  scope VARCHAR(255) NOT NULL,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  redirect_uri VARCHAR(128) NOT NULL,
  user_id INTEGER,
  CONSTRAINT auth_codes__client_id
    FOREIGN KEY (client_id)
    REFERENCES clients (id)
);

INSERT INTO grant_types (name) VALUES
  ('authorization_code'),
  ('token'),
  ('password'),
  ('client_credentials'),
  ('refresh_token');





INSERT INTO clients (identifier, secret, response_type) VALUES
  ('abcd1234', 'abcd1234', 'something');
INSERT INTO client_redirect_uris (client_id, redirect_uri) VALUES
  (1, 'http://localhost/testing/redirect_uri_one');
