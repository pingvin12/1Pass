import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'

export default function Register() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Register</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Register
        </h1>

        <input placeholder='Username'></input>
        <input placeholder='Email'></input>
        <input placeholder='Password'></input>

        <input type={"button"}>Register</input>
      </main>

      <footer className={styles.footer}>
        Made by Jozsef Fenyes
      </footer>
    </div>
  )
}
