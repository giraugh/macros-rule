import Highlight from 'reveal.js/plugin/highlight/highlight'
import Markdown from 'reveal.js/plugin/markdown/markdown'
import MathReveal from 'reveal.js/plugin/math/math'
import RevealNotes from 'reveal.js/plugin/notes/notes'

import 'reveal.js/dist/reveal.css'
import './assets/hljs_equilibrium.css'
import './theme.css'

export default {
	revealOptions: {
		plugins: [Highlight, Markdown, MathReveal.MathJax2, MathReveal.KaTeX, RevealNotes],
		hash: true,
		mathjax2: {
			config: 'TeX-AMS_HTML-full',
			TeX: {
				Macros: {
					R: '\\mathbb{R}',
					set: ['\\left\\{#1 \\; ; \\; #2\\right\\}', 2]
				}
			}
		},
		width: 1300,
		height: 850,
		transition: 'none'
	}
}
