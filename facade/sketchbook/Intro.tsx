import dynamic from 'next/dynamic'
import { Words } from "persona"
import { memory } from "persona/persona_bg.wasm"
import p5Types from "p5"

const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)


const Intro = () => {

    let size: number
    let cell_count: number
    let words: Words
    

    const setup = (p5: p5Types, canvasParentRef: Element) => {

        p5.createCanvas(window.innerWidth, window.innerHeight).parent(canvasParentRef)
        p5.colorMode(p5.HSB)
		p5.background(0)

        words = Words.new(
            0, 
            0,
            window.innerWidth, 
            window.innerHeight, 
            "SOHEIL\n DEVELOPER AND\n SOME OTHER THINGS"
        )

        p5.print([window.innerWidth, 
            window.innerHeight,])

        size = words.get_particle_size() - 1
        cell_count = words.get_cell_count()

        const cellsPtr = words.cells();
		const cells = new Int32Array(memory.buffer, cellsPtr, cell_count);

        p5.noStroke()
        for (let i=0; i < cell_count; i++) {
            p5.rect(cells[i], cells[++i], size, size)
        }
	};


	const draw = (p5: p5Types) => {
        p5.background(0)
		words.run(p5.mouseX, p5.mouseY)

		const cellsPtr = words.cells();
		const cells = new Int32Array(memory.buffer, cellsPtr, cell_count);
        
        p5.noStroke()
        for (let i=0; i < cell_count; i++) {
            p5.rect(cells[i], cells[++i], size, size)
        }
	};


    const mouseClicked = (p5: p5Types) => {
        words.repellent(p5.mouseX, p5.mouseY)

        const cellsPtr = words.cells();
		const cells = new Int32Array(memory.buffer, cellsPtr, cell_count);
        p5.print(cells)
    }


	return <Sketch className={"follow-sketch"} setup={setup} draw={draw} mouseClicked={mouseClicked} />
}


export default Intro