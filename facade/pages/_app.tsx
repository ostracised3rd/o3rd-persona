import '../styles/globals.css'
import { FC, useEffect } from 'react'
import type { AppProps } from 'next/app'
import { Layout } from '@components/Master'

const Noop: FC = ({ children }) => <>{children}</>

export default function MyApp({ Component, pageProps }: AppProps) {
  // const Layout = (Component as any).Layout || Noop

  useEffect(() => {
    document.body.classList?.remove('loading')
  }, [])

  return (
    <>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </>
  )
}
