import React, { useState } from "react";
import { useRouter } from "next/router";
import Image from "next/image";
import logo from "../../assets/img/1pass.png";

function Navbar() {
  const [isExpanded, setExpanded] = useState(false);
  const router = useRouter();

  const handleLinkClick = (path) => {
    setExpanded(false);
    router.push(path);
  };

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
          <li>
            <a onClick={() => handleLinkClick("/login")}>Sign In</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick("/register")}>Sign Up</a>
          </li>
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
