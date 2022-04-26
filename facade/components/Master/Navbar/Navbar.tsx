import Link from 'next/link'
import React from "react";


const Navbar = () => {
  const [navbarOpen, setNavbarOpen] = React.useState(true);
  return (

    <>
      <nav className="relative flex flex-wrap items-center justify-between px-2 py-3 bg-primary border-d border-red-900">
        <div className="container px-4 mx-auto flex flex-wrap items-center justify-between">
          <div className="w-full relative flex justify-between lg:w-auto lg:static lg:block lg:justify-start">
          
            <h1 className="text-xl font-bold leading-relaxed inline-block mr-4 py-2 whitespace-nowrap uppercase text-secondary">
            <Link href='/'>O3rd</Link>
            </h1>
            
            <button
              className="text-secondary cursor-pointer text-xl leading-none px-3 py-1 border border-solid border-transparent rounded bg-transparent block lg:hidden outline-none focus:outline-none"
              type="button"
              onClick={() => setNavbarOpen(!navbarOpen)}
            >
              <i className="fas fa-bars"></i>
            </button>
          </div>
          <div
            className={
              "lg:flex flex-grow items-center" +
              (navbarOpen ? " flex" : " hidden")
            }
            id="example-navbar-danger"
          >
            <ul className="flex flex-col lg:flex-row list-none lg:ml-auto">
              <li className="nav-item">
              <h1 className="px-3 py-2 flex items-center text-xs uppercase font-bold leading-snug text-secondary hover:text-red-400">
                <Link href='/about'>about</Link>
                <i className="fab fa-facebook-square text-lg leading-lg text-white opacity-75"></i>
              </h1>
              </li>

              <li className="nav-item">
              <h1 className="px-3 py-2 flex items-center text-xs uppercase font-bold leading-snug text-white hover:opacity-75">
                <Link href='/drafts/maze-runner'>maze-runner</Link>
                <i className="fab fa-facebook-square text-lg leading-lg text-white opacity-75"></i>
              </h1>
                 
  
              </li>
            </ul>
          </div>
        </div>
      </nav>
    </>
  )
}

export default Navbar
