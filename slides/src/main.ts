import Reveal from 'reveal.js'

import RevealHighlight from 'reveal.js/plugin/highlight/highlight'
import RevealMarkdown from 'reveal.js/plugin/markdown/markdown'

import 'reveal.js/dist/reveal.css'
import 'reveal.js/dist/theme/black.css'
import 'reveal.js/plugin/highlight/monokai.css'

import './global.css'

const deck = new Reveal()
deck.initialize({
  hash: true,
  plugins: [
    RevealMarkdown,
    RevealHighlight,
  ]
})
