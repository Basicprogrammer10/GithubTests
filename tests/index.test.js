const index = require('../index');

// Test the get random name method
test('getRandomName()', () => {
    const name = index.getRandomName();
    expect(name).toBeDefined();
});