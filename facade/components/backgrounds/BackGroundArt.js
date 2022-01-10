
import dynamic from 'next/dynamic'
const Sketch = dynamic(() => import('react-p5').then((mod) => mod.default), {
  ssr: false,
})


const BackGroundArt = () => {
  const setup = (p5, canvasParentRef) => {
		// use parent to render the canvas in this ref
		// (without that p5 will render the canvas outside of your component)
		p5.createCanvas(window.innerWidth, window.innerHeight).parent(canvasParentRef);
	};

	const draw = (p5) => {
		p5.background(0);
		p5.ellipse(window.innerWidth / 2, window.innerHeight / 2, 200, 200);
		// NOTE: Do not use setState in the draw function or in functions that are executed
		// in the draw function...
		// please use normal variables or class properties for these purposes
		p5.noLoop();
	};

	return <Sketch setup={setup} draw={draw} />;
}


let x = 50;
let y = 50;

export default BackGroundArt
