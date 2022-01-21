import Header from './Header'
import Meta from './Meta'
import React from "react"
import Footer from './Footer'

const Layout = ({ children }: {children: React.ReactNode }) => {
  return (
    <>
      <Meta />
      <Header />
      <div className="">
        <main className="">
          {children}
        </main>
      </div>
    </>
  )
}

export default Layout
