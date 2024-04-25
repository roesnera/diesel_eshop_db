-- Your SQL goes here
CREATE TABLE users_roles ( 
    id SERIAL PRIMARY KEY,
    user_id integer NOT NULL references users(id),
    role_id integer NOT NULL references roles(id)
)