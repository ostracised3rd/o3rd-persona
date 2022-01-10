import dynamic from 'next/dynamic';
import { BreadthFirst, DepthFirst } from "persona";
import { memory } from "persona/persona_bg.wasm";

// const bg = dynamic(() => import('persona/persona_bg').then((mod) => mod.default), {
// 	ssr: false,
// })

const Sketch = dynamic(() => import('react-p5').then((mod) => mod.default), {
  ssr: false,
})

// import * as persona from "persona" 


let maze
let matrix
let runner

let size
let mw
let mh
let cell_width
let cell_height
let c = 1

const mazeRunner = () => {
    const setup = (p5, canvasParentRef) => {
		p5.frameRate(30);
		size = window.innerWidth > window.innerHeight ? window.innerHeight : window.innerWidth
		size = size - (size % 100)
		p5.createCanvas(size, size).parent(canvasParentRef)

		mw = 50;
		mh = 50;

		cell_width = Math.floor(size / mw)
		cell_height = Math.floor(size / mh)

		maze = BreadthFirst.new(mw, mh)
		// maze = DepthFirst.new(mw, mh)
		// matrix = maze.matrix()
		// runner = maze.breadth_first()
		// p5.print(matrix, cell_width, cell_height, size)
		p5.colorMode(p5.HSB)
		p5.background(0)

		
		p5.print(maze.get_end())
	};

	const draw = (p5) => {
		p5.print(c++)
		maze.run(c)

		const cellsPtr = maze.cells();
		const cells = new Uint8Array(memory.buffer, cellsPtr, mw * mh);


		for (let row = 0; row < mh; row++) {
			for (let col = 0; col < mw; col++) {
				const idx = getIndex(row, col);

				switch(cells[idx]) {
					case 0:
						p5.fill(100, 0, 0)
						break;
					case c:
						p5.fill(0, 100, 100)
						break;

					default:
						p5.fill(100, 0, 100)
				}

				p5.rect(col * cell_width, row * cell_height, cell_width, cell_height)
			}
		}

		if (maze.is_done()) p5.noLoop()
	};

	const keyPressed = (p5) => {
		// let keyIndex = -1;


		switch(p5.key) {
			case 's':
				p5.loop()
				break;
			case 'Q':
				// code block
				break;
			case 'R':
				// code block
				break;

			case 'f':
				p5.noLoop()
				break;
			default:
				// code block
		}
	}

	return <Sketch setup={setup} draw={draw} keyPressed={keyPressed} />;
}

const drawCells = (p5) => {
	const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
}


const getIndex = (row, column) => {
    return row * mw + column;
};


const drawMatrix = (p5) => {
	// p5.fill(100, 0, 100)
	// p5.noStroke()
	// for (let i=0; i < matrix.length; i++) {
	// 	p5.rect(matrix[i] * cell_width, matrix[++i] * cell_height, cell_width, cell_height)
	// }

	
}

const drawSteps = (p5) => {

	if (runner.is_done()) p5.noLoop()
	
	p5.fill(0, 204, 100)
	runner.run()
	let steps = runner.draw()
	p5.print(steps)
	let done = false
	let count = steps[0];
	let i = 1

	while (!done) {
		for (i; i <= count; i++) {
			p5.rect(steps[i] * cell_width, steps[++i] * cell_height, cell_width, cell_height)
		} 

		if (i >= steps.length) done = true 
		count = steps[++i]
	}
}

export default mazeRunner
