import React from "react";
import { useState } from "react";
import { useRouter } from "next/router";
import Image from "next/image"

function Navbar() {
    const [isExpanded, setExpanded] = useState(false)
    const router = useRouter()
    return (
      <nav className="navigation">
        <a className="Brand name"></a>
        <div
          className={"navigation-menu"}
        >
          <ul>
            <li>
              <a onClick={() => router.push('/')}>Home</a>
            </li>
            <li>
              <a  onClick={() => router.push('/shared')}>Shared items</a>
            </li>
            <li>
              <a onClick={() => router.push('/settings')}>Settings</a>
            </li>
            <li>
              <a onClick={() => router.push('/about')}>
                About
              </a>
            </li>
                <li>
                  <a onClick={(e) => {
                    router.push('/login')
                    }}>Sign In</a>
                </li>
                <li>
                  <a onClick={(e) => {
                    router.push('/register')
                    }}>Sign Up</a>
                </li>
          </ul>
        </div>
      </nav>
    );
}

export default Navbar;