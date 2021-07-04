describe('Test', () => {
  it('should have a passing test', () => {
    cy.visit('localhost:3000')
    expect(true).to.equal(false)
  })
})

describe('EXIT', () => {
    it('Stop server now that tests are done', () => {
      cy.visit('localhost:3000/EXIT')
    })
  })