const drugInteractions = {
    "Paracetamol": ["Ibuprofen"],
    "Ibuprofen": ["Paracetamol", "Aspirin"],
    "Aspirin": ["Ibuprofen"],
    "Cough Syrup": [],
    "Antihistamine": [],
    "Multivitamin": []
};

const handler = async (event) => {
    const medications = event.medications || [];
    
    const interactions = [];
    for (let i = 0; i < medications.length; i++) {
        const med1 = medications[i];
        const med1Interactions = drugInteractions[med1] || [];
        for (let j = i + 1; j < medications.length; j++) {
            const med2 = medications[j];
            if (med1Interactions.includes(med2)) {
                interactions.push([med1, med2]);
            }
        }
    }
    
    if (interactions.length > 0) {
        return {
            statusCode: 200,
            body: JSON.stringify({
                message: 'There are interactions between the medications',
                interactions: interactions,
            }),
        };
    } else {
        return {
            statusCode: 200,
            body: JSON.stringify({
                message: 'No interactions between the medications',
            }),
        };
    }
};

export { handler };
