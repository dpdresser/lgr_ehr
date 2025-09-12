-- Create keycloak schema for Keycloak tables
CREATE SCHEMA IF NOT EXISTS keycloak;

-- Grant privileges to the database user
GRANT ALL PRIVILEGES ON SCHEMA keycloak TO postgres;

-- Set default privileges for future objects in keycloak schema
ALTER DEFAULT PRIVILEGES IN SCHEMA keycloak GRANT ALL ON TABLES TO postgres;
ALTER DEFAULT PRIVILEGES IN SCHEMA keycloak GRANT ALL ON SEQUENCES TO postgres;
ALTER DEFAULT PRIVILEGES IN SCHEMA keycloak GRANT ALL ON FUNCTIONS TO postgres;