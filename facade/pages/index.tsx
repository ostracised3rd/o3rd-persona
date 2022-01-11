import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'

import FollowSketch from '../components/sketch/FollowSketch'
import CubeSketch from '../components/sketch/CubeSketch'

const Home: NextPage = () => {
  return (
    <div className="bg-neutral-800 w-screen h-screen">

      <div className='bg-gray-400 w-96 h-96 relative z-0'>
      {/* background canvas */}
        <div className="w-full h-screen bg-gray-200 flex justify-center items-center follow-sketch ">
          <FollowSketch />
        </div>

        <div className='z-100 w-screen h-screen absolute inset-0 flex justify-center items-center z-10'>
          <h1>Soheil Eivazy</h1>
          <h1>Web developer and some other things</h1>
        </div>
      </div>

{/* <div class="w-full h-screen bg-gray-200 flex justify-center items-center">
  <div class="bg-gray-400 w-96 h-96 relative z-0">
    <p class="italic text-bold bd-red-100 font-serif">Map</p>
    <div class="absolute inset-0 flex justify-center items-center z-10">
      <p class="text-2xl font-bold">This should be on top of the map</p>
    </div>
  </div>
</div> */}
      
      {/* intro */}
      

      {/* about */}
      <div>
        {/* text */}
        <div>
          <h1>about me</h1>
          <p>a cryptid and cosmic horror, sometimes do programming
          a cryptid and cosmic horror, sometimes do programming
          a cryptid and cosmic horror, sometimes do programming
          a cryptid and cosmic horror, sometimes do programming
          a cryptid and cosmic horror, sometimes do programming
          a cryptid and cosmic horror, sometimes do programming
          a cryptid and cosmic horror, sometimes do programming
          </p>
        </div>

        {/* cube canvas */}
        <div className="cube-sketch h-1/2 w-1/2">
          <CubeSketch/>
        </div>
      </div>

      {/* contact */}
      <div></div>

      {/* footer */}
      <div></div>
    </div>
  )
}

export default Home
