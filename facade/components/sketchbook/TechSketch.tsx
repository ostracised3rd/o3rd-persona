import dynamic from 'next/dynamic'
import p5Types from "p5"

const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)

let x = 50;
let y = 50;
const TechSketch = () => {
  const setup = (p5: p5Types, canvasParentRef: Element) => {
		// use parent to render the canvas in this ref
		// (without that p5 will render the canvas outside of your component)
		p5.createCanvas(window.innerWidth / 2, window.innerHeight).parent(canvasParentRef);
		p5.colorMode(p5.HSB)
		p5.background(0, 0, 15)
		p5.translate(p5.width / 2, p5.height / 2)

		p5.stroke(0, 0, 100); // Change the color
		p5.strokeWeight(10);

		p5.fill(0, 0, 100);
		for (let i=0; i < 30; i++) {
			p5.point(x+i, p5.sin(x+i))
		}
	};

	const draw = (p5: p5Types) => {
		// p5.background(0);
		// p5.rect(x, y, 70, 70);
		// NOTE: Do not use setState in the draw function or in functions that are executed
		// in the draw function...
		// please use normal variables or class properties for these purposes
		// x++;
		
	};

	return <Sketch className={"cube-sketch"} setup={setup} draw={draw}/>;
}

export default TechSketch
