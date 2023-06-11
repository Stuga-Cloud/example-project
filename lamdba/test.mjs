import { handler } from './drugs.mjs'

const testEvent = {
    medications: ["Ibuprofen", "Paracetamol"],
};

(async () => {
    const data = JSON.stringify(testEvent);
    const result = await handler({ body: data });
    console.log(result);
})();
