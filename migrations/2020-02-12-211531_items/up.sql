CREATE EXTENSION "uuid-ossp";
CREATE TABLE "items" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "name" VARCHAR NOT NULL,
  "buy_time" timestamp,
  "owner" VARCHAR,
  "create_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "type_id" SMALLINT DEFAULT 0,
  "comment" VARCHAR
);
DO $$                  
    BEGIN 
        IF EXISTS
            ( SELECT 1
              FROM   information_schema.tables 
              WHERE  table_name = 'types'
            )
        THEN
            ALTER TABLE "items"
                ADD FOREIGN KEY ("type_id") REFERENCES "types" ("id");
        END IF ;
    END
   $$ ;