import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { useState, useEffect } from 'react'
export default function App({ Component, pageProps }: AppProps) {
  const [loading, setLoading] = useState(false);
  const Letters = [ 'R', 'O', 'D', 'E', 'O' ];
  const moving = require('react-moving-text');
  useEffect(() => {
    setLoading(true)
    setTimeout(() => { setLoading(false) }, 99999999)
  }, [])
  return (<>
  {
    loading ?
    Letters.map((letter, index) =>
      <moving.MovingComponent
        type="effect3D"
        duration="2000ms"
        delay="index * 100ms"
        direction="reverse"
        timing="ease"
        iteration="infinite"
        fillMode="both">
        {letter}
      </moving.MovingComponent>)
  :
  <Component {...pageProps} />
  }
  </>
  )
}
