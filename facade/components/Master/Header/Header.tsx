import Head from 'next/head'

const Header = ({ title, keywords, description }: {title: string, keywords: string, description: string}) => {
  return (
    <Head>
      <meta name='viewport' content='width=device-width, initial-scale=1' />
      <meta name='keywords' content={keywords} />
      <meta name='description' content={description} />
      <meta charSet='utf-8' />
      <link rel='icon' href='/favicon.ico' />
      <title>{title}</title>
    </Head>
  )
}

Header.defaultProps = {
  title: 'o3rd',
  keywords: 'web development, programming, generative art, ',
  description: 'random',
}

export default Header
