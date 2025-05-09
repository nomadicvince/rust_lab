-- Define the note table with schema validation
DEFINE TABLE note SCHEMAFULL;

-- Define fields with types
DEFINE FIELD title ON TABLE note TYPE string;
DEFINE FIELD content ON TABLE note TYPE string;
DEFINE FIELD created_at ON TABLE note TYPE datetime;
DEFINE FIELD updated_at ON TABLE note TYPE datetime;
DEFINE FIELD created_by ON TABLE note TYPE string;
DEFINE FIELD is_shared ON TABLE note TYPE bool;
DEFINE FIELD shared_with ON TABLE note TYPE array;
DEFINE FIELD version ON TABLE note TYPE number;

-- Define indexes for efficient queries
DEFINE INDEX note_creator ON TABLE note FIELDS created_by;
DEFINE INDEX note_updated ON TABLE note FIELDS updated_at;

-- Define the user table (for future auth)
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string;
DEFINE FIELD display_name ON TABLE user TYPE string;

-- Define unique indexes for user
DEFINE INDEX user_username ON TABLE user FIELDS username UNIQUE;
DEFINE INDEX user_email ON TABLE user FIELDS email UNIQUE;