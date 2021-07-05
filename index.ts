function getRandomName(): string {
    let items = [
        'Jeff',
        'Jeb',
        'Bill',
        'Bob',
        'Buddy',
        'James',
        'Jim',
        'Jimmy',
        'Joe',
        'Joey',
        'Johnny',
        'Billie',
        'Billy'
    ];
    return items[Math.floor(Math.random() * items.length)];
}

console.log(`Hello, ${getRandomName()}`);

module.exports = {
    getRandomName: getRandomName
};
