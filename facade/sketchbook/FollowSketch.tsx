import dynamic from 'next/dynamic'
import p5Types from "p5"

const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)

let x = 50;
let y = 50;
const FollowSketch = () => {
  const setup = (p5: p5Types, canvasParentRef: Element) => {

		p5.createCanvas(window.innerWidth, window.innerHeight).parent(canvasParentRef)
		p5.colorMode(p5.HSB)
		p5.background(0, 0, 15);
	};

	// const draw = (p5: p5Types) => {
	// 	p5.background(100);
	// 	p5.ellipse(x, y, 70, 70);
	// 	// NOTE: Do not use setState in the draw function or in functions that are executed
	// 	// in the draw function...
	// 	// please use normal variables or class properties for these purposes
	// 	x++;
	// };

	return <Sketch className={"follow-sketch"} setup={setup} />;
}

export default FollowSketch
