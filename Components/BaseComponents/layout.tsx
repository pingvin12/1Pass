import Head from 'next/head'
import Image from 'next/image'
import React, { ReactNode } from 'react'

export default function Layout({ children }: {children: ReactNode}) {
  return (
    <div className="rounded-xl bg-white bg-opacity-50 px-16 py-10 shadow-lg backdrop-blur-md max-sm:px-8">
      <div className="dark:text-white text-black">
        {children}
      </div>
    </div>
  )
}

