import React, { useContext, useEffect, useState } from "react";
import { useRouter } from "next/router";
import Image from "next/image";
import logo from "../../assets/img/1pass.png";
import { logoutAsync, me } from "../../pages/api/auth/auth";

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

  const logout = async () => {
    if (typeof window !== "undefined") {
      await logoutAsync();
      setUserData({
        name: "",
        email: "",
        jwtToken: {
          token: "",
          validity: 0,
        },
      });
      
      router.push("/");
      }
  }
  

  useEffect(() => {
    const fetchData = async () => {

      const token: string | null = localStorage.getItem("token");
      if (typeof window !== "undefined" && token !== null) {
        const user_object = await me(token as string);
        if (user_object !== undefined) {
          const user = JSON.parse(user_object);
          if (user !== undefined && token) {
          setUserData({
            name: user.username,
            email: user.email,
            jwtToken: {
              token: token,
              validity: user.exp,
            }
          })
        }
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
            <a onClick={() => handleLinkClick("/profile")}>{userData.name}&apos;s Profile</a>
          </li>
          <li>
            <a onClick={() => logout()}>Logout</a>
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
