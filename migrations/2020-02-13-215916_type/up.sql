CREATE TABLE "types" (
  "id" SMALLSERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "create_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "comment" VARCHAR
);
INSERT INTO "types" ("id", "name", "comment") VALUES(0, 'Default', 'Automatically created by bb.');
DO $$                  
    BEGIN 
        IF EXISTS
            ( SELECT 1
              FROM   information_schema.tables 
              WHERE  table_name = 'items'
            )
        THEN
            ALTER TABLE "items"
                ADD FOREIGN KEY ("type_id") REFERENCES "types" ("id");
        END IF ;
    END
   $$ ;