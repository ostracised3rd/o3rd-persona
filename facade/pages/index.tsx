import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import Footer from '../components/Footer'

import CliffordAttractor from '../sketchbook/CliffordAttractor'
import TechSketch from '../sketchbook/TechSketch'

const Home: NextPage = () => {
  return (
    <div className="bg-primary w-screen h-screen font-Josefin">

      <div className='w-screen h-screen relative z-0'>
        {/* background canvas */}
        <div className="w-screen h-screen flex justify-center items-center follow-sketch ">
          <CliffordAttractor />
        </div>

        {/* intro */}
        <div className='w-screen h-screen absolute inset-0 flex justify-center items-center z-10 text-accent font-Josefin'>
          <div className='text-3xl'>
            <h1>Soheil Eivazy</h1>
            <h1>Web developer and some other things</h1>
          </div>
        </div>
      </div>

      {/* about */}
      <div className='w-screen h-screen bg-primary flex'>
        {/* text */}
        <div className='h-screen w-1/2 p-3 text-secondary flex-col text-center'>
          <div><h1 className='py-2 text-xl'>about me</h1></div>
          <div>
            <p className='flex-row col-2 px-10'>a cryptid and cosmic horror, sometimes do programming
              a cryptid and cosmic horror, sometimes do programming
              a cryptid and cosmic horror, sometimes do programming
              a cryptid and cosmic horror, sometimes do programming
              a cryptid and cosmic horror, sometimes do programming
              a cryptid and cosmic horror, sometimes do programming
              a cryptid and cosmic horror, sometimes do programming
            </p>
          </div>
        </div>

        {/* cube canvas */}
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
      <div>
        <section className="bg-primary text-secondary body-font relative">
          <div className="container px-5 py-24 mx-auto">
            <div className="flex flex-col text-center w-full mb-12">
              <h1 className="sm:text-3xl text-2xl font-medium title-font mb-2 text-accent">
                Contact Me
              </h1>
            </div>
            <div className="lg:w-1/2 md:w-2/3 mx-auto">
              <div className="flex flex-wrap -m-2">
                <div className="p-2 w-1/2">
                  <div className="relative">
                    <label htmlFor="name" className="leading-7 text-sm text-secondary">
                      Name
                    </label>
                    <input
                      type="text"
                      id="name"
                      name="name"
                      className="w-full bg-secondary-dark rounded border border-gray-300 focus:border-primary-light text-primary-dark outline-none text-primary-dark py-1 px-3 leading-8 transition-colors duration-200 ease-in-out"
                    />
                  </div>
                </div>
                <div className="p-2 w-1/2">
                  <div className="relative">
                    <label
                      htmlFor="email"
                      className="leading-7 text-sm text-gray-100"
                    >
                      Email
                    </label>
                    <input
                      type="email"
                      id="email"
                      name="email"
                      className="w-full bg-gray-100 rounded border border-gray-300 focus:border-indigo-500 text-base outline-none text-primary-dark py-1 px-3 leading-8 transition-colors duration-200 ease-in-out"
                    />
                  </div>
                </div>
                <div className="p-2 w-full">
                  <div className="relative">
                    <label
                      htmlFor="message"
                      className="leading-7 text-sm text-gray-100"
                    >
                      Message
                    </label>
                    <textarea
                      id="message"
                      name="message"
                      className="w-full bg-gray-100 rounded border border-gray-300 focus:border-indigo-500 h-32 text-base outline-none text-gray-700 py-1 px-3 resize-none leading-6 transition-colors duration-200 ease-in-out"
                    ></textarea>
                  </div>
                </div>
                <div className="p-2 w-full">
                  <button className="flex mx-auto text-neutral-800 bg-yellow-400 border-0 py-2 px-8 focus:outline-none hover:bg-red-500 rounded text-lg">
                    Send Message
                  </button>
                </div>
              </div>
            </div>
          </div>
        </section>
        <Footer/>
      </div>
    </div >
  )
}

export default Home
