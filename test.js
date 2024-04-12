const { renderHighlightedString } = require('omorphia')
const fs = require('node:fs');

const text = fs.readFileSync(process.argv[2], 'utf8')
renderHighlightedString(text)