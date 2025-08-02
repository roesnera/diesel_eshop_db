-- Your SQL goes here
CREATE TABLE items_images (
    id SERIAL PRIMARY KEY,
    item_id INT NOT NULL references items(id),
    image_id INT NOT NULL references images(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);