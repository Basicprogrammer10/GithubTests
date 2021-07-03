function getRandomName() {
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
  ]
  return undefined///items[Math.floor(Math.random()*items.length)]
}

console.log(`Hello, ${getRandomName()}`)

module.exports = {
  getRandomName: getRandomName
}
