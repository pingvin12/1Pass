
import type { AppProps } from 'next/app'
import { useState, useEffect } from 'react'
import { SessionProvider } from "next-auth/react"
import type { Session } from "next-auth"
import '../styles/globals.css'
import Navbar from '../Components/navbar'
export default function App({ Component, pageProps: { session, ...pageProps} }: AppProps<{ session: Session}>) {
  return (
    <SessionProvider session={session}>
    <Navbar/>
  <Component {...pageProps} />
  </SessionProvider>
  )
}
