import React, { useContext, useEffect, useState } from "react";
import { useRouter } from "next/router";
import Image from "next/image";
import logo from "../../assets/img/1pass.png";
import { loginAsync, logoutAsync, me } from "../../pages/api/auth/auth";
import useSWR, { useSWRConfig } from "swr";
import { authObject } from "../../pages/api/models/auth.object";
import { invokeFetcher } from "../../pages/api/invoke/useInvoke";

function Navbar() {
  const [isExpanded, setExpanded] = useState(false);
  const [loggedIn, setLoggedIn] = useState(false);
  const [userData, setUserData] = useState({
    name: "",
    email: "",
    jwtToken: {
      token: "",
      validity: 0,
    },
  });
  const router = useRouter();

  const handleLinkClick = (path) => {
    setExpanded(false);
    router.push(path);
  };

  const logout = async (token: string) => {

  }

  useEffect(() => {
    const fetchData = async () => {
      if (typeof window !== "undefined") {
        const authtoken: string | null = localStorage.getItem("token");
        console.log(authtoken);
        const user = JSON.parse(await me(authtoken as string) ?? "");
        if (user !== undefined && authtoken) {
          console.log(user);
          setUserData({
            name: user.username,
            email: user.email,
            jwtToken: {
              token: authtoken,
              validity: user.exp,
            }
          })
        }
        
      }
    };

    const checkLoggedIn = () => {
      if (typeof window !== "undefined") {
        fetchData();
      }
    };

    router.events.on("routeChangeComplete", checkLoggedIn);
    
    return () => {
      router.events.off("routeChangeComplete", checkLoggedIn);
    };
  }, [router.events, loggedIn]);

  return (
    <nav className="navigation">
      <div className={`navigation-menu ${isExpanded ? "expanded" : ""}`}>
        <ul className="nav-links">
          <Image
            src={logo}
            width={50}
            height={50}
            alt="logo"
            className="navbar-logo"
          />
          <li>
            <a onClick={() => handleLinkClick("/")}>Home</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick("/shared")}>Items</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick("/settings")}>Settings</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick("/about")}>About</a>
          </li>
          {
            userData.name !== "" ? 
            <>
            <li>
            <a onClick={() => handleLinkClick("/profile")}>{userData.name}'s Profile</a>
          </li>
          <li>
            <a onClick={() => logout(userData.jwtToken.token)}>Logout</a>
          </li>
            </>
            :
            <>
            <li>
            <a onClick={() => handleLinkClick("/login")}>Sign In</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick("/register")}>Sign Up</a>
          </li>
            </>
          }
          
        </ul>
        <button
          className={`hamburger ${isExpanded ? "expanded" : ""}`}
          onClick={() => setExpanded(!isExpanded)}
        >
          <span></span>
          <span></span>
          <span></span>
        </button>
      </div>
    </nav>
  );
}

export default Navbar;
