import Link from 'next/link'
import React from "react";


const Nav = () => {
  const [navbarOpen, setNavbarOpen] = React.useState(true);
  return (

    <>
      <nav className="relative flex flex-wrap items-center justify-between px-2 py-3 bg-neutral-900 border-d border-red-900">
        <div className="container px-4 mx-auto flex flex-wrap items-center justify-between">
          <div className="w-full relative flex justify-between lg:w-auto lg:static lg:block lg:justify-start">
          
            <h1 className="text-xl font-bold leading-relaxed inline-block mr-4 py-2 whitespace-nowrap uppercase text-yellow-400">
            <Link href='/'>O3rd</Link>
            </h1>
            
            <button
              className="text-white cursor-pointer text-xl leading-none px-3 py-1 border border-solid border-transparent rounded bg-transparent block lg:hidden outline-none focus:outline-none"
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
              <h1 className="px-3 py-2 flex items-center text-xs uppercase font-bold leading-snug text-yellow-400 hover:text-red-400">
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

export default Nav



// import React from "react";
// import { createPopper } from "@popperjs/core";

// const Dropdown = ({ color }) => {
//   // dropdown props
//   const [dropdownPopoverShow, setDropdownPopoverShow] = React.useState(false);
//   const btnDropdownRef = React.createRef();
//   const popoverDropdownRef = React.createRef();
//   const openDropdownPopover = () => {
//     createPopper(btnDropdownRef.current, popoverDropdownRef.current, {
//       placement: "bottom-start"
//     });
//     setDropdownPopoverShow(true);
//   };
//   const closeDropdownPopover = () => {
//     setDropdownPopoverShow(false);
//   };
//   // bg colors
//   let bgColor;
//   color === "white"
//     ? (bgColor = "bg-blueGray-700")
//     : (bgColor = "bg-" + color + "-500");
//   return (
//     <>
//       <div className="flex flex-wrap">
//         <div className="w-full sm:w-6/12 md:w-4/12 px-4">
//           <div className="relative inline-flex align-middle w-full">
//             <button
//               className={
//                 "text-white font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150 " +
//                 bgColor
//               }
//               type="button"
//               ref={btnDropdownRef}
//               onClick={() => {
//                 dropdownPopoverShow
//                   ? closeDropdownPopover()
//                   : openDropdownPopover();
//               }}
//             >
//               {color === "white" ? "White Dropdown" : color + " Dropdown"}
//             </button>
//             <div
//               ref={popoverDropdownRef}
//               className={
//                 (dropdownPopoverShow ? "block " : "hidden ") +
//                 (color === "white" ? "bg-white " : bgColor + " ") +
//                 "text-base z-50 float-left py-2 list-none text-left rounded shadow-lg mt-1"
//               }
//               style={{ minWidth: "12rem" }}
//             >
//               <a
//                 href="#pablo"
//                 className={
//                   "text-sm py-2 px-4 font-normal block w-full whitespace-nowrap bg-transparent " +
//                   (color === "white" ? " text-blueGray-700" : "text-white")
//                 }
//                 onClick={e => e.preventDefault()}
//               >
//                 Action
//               </a>
//               <a
//                 href="#pablo"
//                 className={
//                   "text-sm py-2 px-4 font-normal block w-full whitespace-nowrap bg-transparent " +
//                   (color === "white" ? " text-blueGray-700" : "text-white")
//                 }
//                 onClick={e => e.preventDefault()}
//               >
//                 Another action
//               </a>
//               <a
//                 href="#pablo"
//                 className={
//                   "text-sm py-2 px-4 font-normal block w-full whitespace-nowrap bg-transparent " +
//                   (color === "white" ? " text-blueGray-700" : "text-white")
//                 }
//                 onClick={e => e.preventDefault()}
//               >
//                 Something else here
//               </a>
//               <div className="h-0 my-2 border border-solid border-t-0 border-blueGray-800 opacity-25" />
//               <a
//                 href="#pablo"
//                 className={
//                   "text-sm py-2 px-4 font-normal block w-full whitespace-nowrap bg-transparent " +
//                   (color === "white" ? " text-blueGray-700" : "text-white")
//                 }
//                 onClick={e => e.preventDefault()}
//               >
//                 Seprated link
//               </a>
//             </div>
//           </div>
//         </div>
//       </div>
//     </>
//   );
// };

// export default function DropdownRender() {
//   return (
//     <>
//       <Dropdown color="teal" />
//     </>
//   );
// }
