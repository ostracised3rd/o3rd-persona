import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'

import { Layout } from '@components/Master'
import { AboutMe, ContactMe } from '@components/Landing'
import { IntroSketch, TechSketch } from '@components/sketchbook'

// import { Intro, TechSketch} from '@sketchbook'

// import Intro from '../sketchbook/Intro'
// import TechSketch from '../sketchbook/TechSketch'

const Home: NextPage = () => {
  return (
    <div className="bg-primary w-screen h-screen font-Josefin">

      <div className="w-screen h-screen flex justify-center items-center follow-sketch ">
        <IntroSketch />
      </div>

      {/* about */}
      <div className='w-screen h-screen bg-primary flex'>
        <AboutMe />

        <div className="cube-sketch h-screen w-1/2">
          <TechSketch />
        </div>

      </div>

      {/* project links */}
      <div>
        {/* maze runner */}
        <div></div>

        {/* rubik's cube  */}
        <div></div>
      </div>

      {/* contact */}
      <ContactMe />
    </div >
  )
}

export default Home
