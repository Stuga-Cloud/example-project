-- Add up migration script here
CREATE TABLE products (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    href VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    description TEXT NOT NULL,
    image_src VARCHAR(255) NOT NULL,
    image_alt VARCHAR(255) NOT NULL
);

INSERT INTO products (id, name, href, price, description, image_src, image_alt) VALUES
    (1, 'Paracetamol', '#', 9.99, 'Relieves pain and reduces fever', 'medecin_1.png', 'medecine fiole'),
    (2, 'Ibuprofen', '#', 12.99, 'Effective for reducing inflammation and pain', 'medecin_2.png', 'medecine fiole'),
    (3, 'Cough Syrup', '#', 6.99, 'Provides relief from cough and congestion', 'medecin_3.png', 'medecine fiole'),
    (4, 'Antihistamine', '#', 8.99, 'Helps relieve allergy symptoms', 'medecin_4.png', 'medecine fiole'),
    (5, 'Multivitamin', '#', 14.99, 'Provides essential vitamins and minerals', 'medecin_5.png', 'medecine fiole'),
    (6, 'Aspirin', '#', 7.99, 'Used for pain relief and to reduce the risk of heart attack and stroke', 'medecin_6.png', 'medecine fiole'),
    (7, 'Headache Relief Pills', '#', 9.99, 'Effective pills for relieving headaches', 'medecin_7.png', 'medecine fiole'),
    (8, 'Allergy Relief Spray', '#', 12.99, 'Fast-acting spray for relieving allergy symptoms', 'medecin_8.png', 'medecine fiole'),
    (9, 'Cold & Flu Pack', '#', 19.99, 'Includes various medications for cold and flu symptoms', 'medecin_pack.png', 'medecine fiole');
