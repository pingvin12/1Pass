import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'

export default function Login() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Login</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Login
        </h1>

        <input placeholder='Username'></input>
        <input placeholder='Password'></input>

        <input type={"button"}>Login</input>
      </main>

      <footer className={styles.footer}>
        Made by Jozsef Fenyes
      </footer>
    </div>
  )
}
