import dynamic from 'next/dynamic'
import p5Types from "p5"

const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)

let x = 0.1
let y = -0.1
let a = 4.2
let b = 2
let c = 1.0
let d = -1.0

a=-1.5 
b=1.8
c=1.4 
d=0.9


const about = () => {
    
    const setup = (p5: p5Types, canvasParentRef: Element) => {
		p5.createCanvas(window.innerWidth, window.innerHeight).parent(canvasParentRef)
		p5.colorMode(p5.HSB)
		p5.background(0, 0, 15)

        p5.stroke(0, 100, 100)
        p5.strokeWeight(1)
		p5.translate(p5.width / 2, p5.height /2)
		p5.stroke(0, 0, 100 )

		for (let i=0; i < 100000; i++) {
			let xn = p5.sin(a*y) + (c * p5.cos(a*x))
			let yn = p5.sin(b*x) + (d * p5.cos(b*y))
			x = xn
        	y = yn
			p5.point(x * 120, y * 120)
		}

		// p5.print("done")

	}

	const draw = (p5: p5Types) => {
		p5.translate(p5.width / 2, p5.height /2)
        let xn = p5.sin(a*y) + (c * p5.cos(a*x))
        let yn = p5.sin(b*x) + (d * p5.cos(b*y))

        // p5.print([xn, yn])
        
        x = xn
        y = yn

		p5.point(x * 100, y * 100)
	}

    const mouseClicked = (p5: p5Types) => {
        p5.translate(p5.mouseX, p5.mouseY)
    }

	return <Sketch className={"follow-sketch"} setup={setup}  mouseClicked={mouseClicked}/>;
}

export default about
