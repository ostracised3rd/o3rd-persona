import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'

import FollowSketch from '../sketchbook/FollowSketch'
import TechSketch from '../sketchbook/TechSketch'

const Home: NextPage = () => {
  return (
    <div className="bg-neutral-800 w-screen h-screen">

      <div className='w-screen h-screen relative z-0'>
        {/* background canvas */}
        <div className="w-screen h-screen flex justify-center items-center follow-sketch ">
          <FollowSketch />
        </div>

        {/* intro */}
        <div className='w-screen h-screen absolute inset-0 flex justify-center items-center z-10 text-white'>
          <div className='text-3xl'>
            <h1>Soheil Eivazy</h1>
            <h1>Web developer and some other things</h1>
          </div>
        </div>
      </div>

      
      
      

      {/* about */}
      <div className='w-screen h-screen bg-neutral-800 flex'>
        {/* text */}
        <div className='h-screen w-1/2 p-3 text-white flex-col text-center'>
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
          <TechSketch/>
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
        <section className="bg-neutral-800 text-gray-100 body-font relative">
          <div className="container px-5 py-24 mx-auto">
            <div className="flex flex-col text-center w-full mb-12">
              <h1 className="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-100">
                Contact Me
              </h1>
              <p className="lg:w-2/3 mx-auto leading-relaxed text-base">
                Whatever cardigan tote bag tumblr hexagon brooklyn asymmetrical
                gentrify.
              </p>
            </div>
            <div className="lg:w-1/2 md:w-2/3 mx-auto">
              <div className="flex flex-wrap -m-2">
                <div className="p-2 w-1/2">
                  <div className="relative">
                    <label htmlFor="name" className="leading-7 text-sm text-gray-100">
                      Name
                    </label>
                    <input
                      type="text"
                      id="name"
                      name="name"
                      className="w-full bg-gray-100 rounded border border-gray-300 focus:border-indigo-500 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out"
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
                      className="w-full bg-gray-100 rounded border border-gray-300 focus:border-indigo-500 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out"
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
                    Button
                  </button>
                </div>
                
              </div>
            </div>
          </div>
        </section>
      </div>

      {/* footer */}
      <div>
      <div className="bg-neutral-900 p-2 w-full pt-3  text-center flex-col">
        <div>
        <a className="text-yellow-400 leading-normal  ">example@email.com</a>
        </div>
        
        <div className='py-3'>
        <span className="inline-flex">
              <a className="text-yellow-400 hover:text-red-500">
                <svg
                  fill="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  className="w-5 h-5"
                  viewBox="0 0 24 24"
                >
                  <path d="M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z"></path>
                </svg>
              </a>
              <a className="ml-4 text-yellow-400 hover:text-red-500">
                <svg
                  fill="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  className="w-5 h-5"
                  viewBox="0 0 24 24"
                >
                  <path d="M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z"></path>
                </svg>
              </a>
              <a className="ml-4 text-yellow-400 hover:text-red-500">
                <svg
                  fill="none"
                  stroke="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  className="w-5 h-5"
                  viewBox="0 0 24 24"
                >
                  <rect
                    width="20"
                    height="20"
                    x="2"
                    y="2"
                    rx="5"
                    ry="5"
                  ></rect>
                  <path d="M16 11.37A4 4 0 1112.63 8 4 4 0 0116 11.37zm1.5-4.87h.01"></path>
                </svg>
              </a>
              <a className="ml-4 text-yellow-400 hover:text-red-500">
                <svg
                  fill="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  className="w-5 h-5"
                  viewBox="0 0 24 24"
                >
                  <path d="M21 11.5a8.38 8.38 0 01-.9 3.8 8.5 8.5 0 01-7.6 4.7 8.38 8.38 0 01-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 01-.9-3.8 8.5 8.5 0 014.7-7.6 8.38 8.38 0 013.8-.9h.5a8.48 8.48 0 018 8v.5z"></path>
                </svg>
              </a>
            </span>
            </div>
            </div>
        </div>
      </div>
  )
}

export default Home
