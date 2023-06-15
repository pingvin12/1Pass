import type { AppProps } from "next/app";
import type { Session } from "next-auth";
import "../styles/globals.css";
import Navbar from "../Components/BaseComponents/navbar";
import Splash from "../Components/BaseComponents/splash";

export default function App({
  Component,
  pageProps: { session, ...pageProps },
}: AppProps<{ session: Session }>) {
  return (
    <Splash interval={1000}>
      <Navbar />
      <Component {...pageProps} />
    </Splash>
  );
}
