import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { useState, useEffect } from 'react'
import LoadingPage from '../Components/LoadingPage';
export default function App({ Component, pageProps }: AppProps) {
  const [loading, setLoading] = useState(false);
  useEffect(() => {
    setLoading(true)
    setTimeout(() => { setLoading(false) }, 10000)
  }, [])
  return (
  <>
  {
    loading ?
    <LoadingPage/>
  :
  <Component {...pageProps} />
  }
  </>
  )
}
