-- Add up migration script here

CREATE TABLE warehouses (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    latitude DECIMAL(9, 6),
    longitude DECIMAL(9, 6)
);

CREATE TABLE products (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    href VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    description TEXT NOT NULL,
    image_src VARCHAR(255) NOT NULL,
    image_alt VARCHAR(255) NOT NULL,
    warehouse_id INT REFERENCES warehouses(id)
);

INSERT INTO warehouses (id, name, location, latitude, longitude) VALUES
    (1, 'Entrepôt Central', '123 Rue Centrale', 48.856613, 2.352222),
    (2, 'Entrepôt Ouest', '456 Rue de l''Ouest', 47.658780, -117.426047),
    (3, 'Entrepôt Est', '789 Rue de l''Est', 37.774929, -122.419416);

INSERT INTO products (id, name, href, price, description, image_src, image_alt, warehouse_id) VALUES
    (1, 'Paracetamol', '#', 9.99, 'Relieves pain and reduces fever', 'medecin_1.png', 'medecine fiole', 1),
    (2, 'Ibuprofen', '#', 12.99, 'Effective for reducing inflammation and pain', 'medecin_2.png', 'medecine fiole', 2),
    (3, 'Cough Syrup', '#', 6.99, 'Provides relief from cough and congestion', 'medecin_3.png', 'medecine fiole', 3),
    (4, 'Antihistamine', '#', 8.99, 'Helps relieve allergy symptoms', 'medecin_4.png', 'medecine fiole', 1),
    (5, 'Multivitamin', '#', 14.99, 'Provides essential vitamins and minerals', 'medecin_5.png', 'medecine fiole', 2),
    (6, 'Aspirin', '#', 7.99, 'Used for pain relief and to reduce the risk of heart attack and stroke', 'medecin_6.png', 'medecine fiole', 3),
    (7, 'Headache Relief Pills', '#', 9.99, 'Effective pills for relieving headaches', 'medecin_7.png', 'medecine fiole', 1),
    (8, 'Allergy Relief Spray', '#', 12.99, 'Fast-acting spray for relieving allergy symptoms', 'medecin_8.png', 'medecine fiole', 2),
    (9, 'Cold & Flu Pack', '#', 19.99, 'Includes various medications for cold and flu symptoms', 'medecin_pack.png', 'medecine fiole', 3);
