
import React from "react"

import { Header, Footer, Navbar} from '@components/Master'

const Layout = ({ children }: {children: React.ReactNode }) => {
  return (
    <>
      <Header />
      <Navbar />
      <div className="">
        <main className="">
          {children}
        </main>
      </div>
      <Footer />
    </>
  )
}

export default Layout
