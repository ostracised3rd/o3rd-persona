import Nav from './Nav'
import Meta from './Meta'
import Header from './Header'
import React from "react"

const Layout = ({ children }: {children: React.ReactNode }) => {
  return (
    <>
      <Meta />
      <Nav />
      <div className="">
        <main className="">
          <Header />
          {children}
        </main>
      </div>
    </>
  )
}

export default Layout
