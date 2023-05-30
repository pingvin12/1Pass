import React, { useState } from "react";
import { useRouter } from "next/router";
import Image from "next/image";
import logo from "../../assets/img/1pass.png"

function Navbar() {
  const [isExpanded, setExpanded] = useState(false);
  const router = useRouter();

  const handleLinkClick = (path) => {
    setExpanded(false);
    router.push(path);
  };

  return (
    <nav className="navigation">
      <Image src={logo} width={50} height={50} alt="logo" className="navbar-logo" />
      <div className={`navigation-menu ${isExpanded ? 'expanded' : ''}`}>
        <ul>
          <li>
            <a onClick={() => handleLinkClick('/')}>Home</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick('/shared')}>Shared Items</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick('/settings')}>Settings</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick('/about')}>About</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick('/login')}>Sign In</a>
          </li>
          <li>
            <a onClick={() => handleLinkClick('/register')}>Sign Up</a>
          </li>
        </ul>
      </div>
      <button className={`hamburger ${isExpanded ? 'expanded' : ''}`} onClick={() => setExpanded(!isExpanded)}>
        <span></span>
        <span></span>
        <span></span>
      </button>
    </nav>
  );
}

export default Navbar;
