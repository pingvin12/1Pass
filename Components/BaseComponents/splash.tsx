import { useEffect, useState } from "react";
import Image from "next/image";
import logo from "../../assets/img/1pass.png";
import SyncLoader from "react-spinners/SyncLoader";
import { motion } from "framer-motion";

interface SplashProps {
  interval: number;
  children: React.ReactNode;
}

const Splash = ({ children, interval }: SplashProps): JSX.Element | null => {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setTimeout(() => {
      setLoading(false);
    }, interval);
  }, []);

  return !loading ? (
    <>{children}</>
  ) : (
    <motion.div
      initial={{ x: 300, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: 300, opacity: 0 }}
      transition={{
        type: "spring",
        duration: 2000,
        stiffness: 260,
        damping: 20,
      }}
    >
      <div className="splash">
        <Image src={logo} alt="logo" width={100} height={100} />
        <h1>Welcome to 1Pass!</h1>
        <SyncLoader size={3} aria-label="Loading Spinner" data-testid="loader" />
      </div>
    </motion.div>
  );
};

export default Splash;
