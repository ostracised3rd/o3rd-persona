import dynamic from 'next/dynamic'
import { Typewriter } from "persona"
import { memory } from "persona/persona_bg.wasm"
import p5Types from "p5"

const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)


const Intro = () => {

    let size: number
    let particle_count: number
    let typewriter: Typewriter
    

    const setup = (p5: p5Types, canvasParentRef: Element) => {

        p5.createCanvas(window.innerWidth, window.innerHeight).parent(canvasParentRef)
        p5.colorMode(p5.HSB)
		p5.background(0)

        typewriter = Typewriter.new(
            0, 0,
            window.innerWidth, 
            window.innerHeight, 
            "SOHEIL\n DEVELOPER AND\n SOME OTHER THINGS"
        )

        size = typewriter.get_size()
        particle_count = typewriter.get_count() * 2

        const cellsPtr = typewriter.cells();
		const cells = new Uint16Array(memory.buffer, cellsPtr, particle_count);

        p5.print([size, particle_count,cells])

        for (let i=0; i < particle_count; i++) {
            p5.rect(cells[i], cells[++i], size, size)
        }
	};


	const draw = (p5: p5Types) => {
		typewriter.run(p5.mouseX, p5.mouseY)

		const cellsPtr = typewriter.cells();
		const cells = new Uint16Array(memory.buffer, cellsPtr, particle_count);
        
        for (let i=0; i < particle_count; i++) {
            
            p5.rect(cells[i], cells[++i], size, size)
        }
	};


    const mouseClicked = (p5: p5Types) => {
        typewriter.repellant(p5.mouseX, p5.mouseY)
    }


	return <Sketch className={"follow-sketch"} setup={setup}  mouseClicked={mouseClicked} />
}


export default Intro