import type { AppProps } from "next/app";
import { useState, useEffect } from "react";
import { SessionProvider } from "next-auth/react";
import type { Session } from "next-auth";
import "../styles/globals.css";
import Navbar from "../Components/BaseComponents/navbar";
import Splash from "../Components/BaseComponents/splash";
import { AnimatePresence } from "framer-motion";

export default function App({
  Component,
  pageProps: { session, ...pageProps },
}: AppProps<{ session: Session }>) {
  return (
    <SessionProvider session={session}>
      <Splash interval={10000}>
        <Navbar />
        <Component {...pageProps} />
      </Splash>
    </SessionProvider>
  );
}
