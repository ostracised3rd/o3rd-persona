import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Index.module.css'

import BackGroundArt from '../components/backgrounds/BackGroundArt'

export default function Home() {
  
  return (
    <div>
      <div className={styles.bgArt} >
        <BackGroundArt />
      </div>
      <div className={styles.page} >
      <div className={styles.welcome} >
        <h1 className={styles.welcomeText}>Soheil Eivazy, Web Developer, and some other thing</h1>
      </div>
      <div className={styles.obj}>
        <h2>about me</h2>
        <p>a cryptid and cosmic horror, sometimes do programming</p>

        <h2>languages</h2>
        <p>python, rust, javascript, go</p>

        <h2>experiences</h2>
        <p>Nutash: android and backend development</p>
        <p></p>
      </div>
      </div>
    </div>
  )
}
