import test from 'ava'
import { optimize } from '..'

test('basic', (t) => {
  const svg = `
<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0, 0, 20, 20">
	<rect width="20" height="20" fill="rgba(255,255,255,.85)" rx="20"/>
</svg>
`
  const output = optimize(svg)
  t.snapshot(output)
})
