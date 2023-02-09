import React from "react";
import { useState } from "react";
import { useRouter } from "next/router";
import { signIn, signOut, useSession } from "next-auth/react";
import Image from "next/image"

function Navbar() {
    const [isExpanded, setExpanded] = useState(false)
    const router = useRouter()
    const {data: session, status } = useSession()
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
            {
              !session && (
                <>
                <li>
                  <a onClick={(e) => {
                    e.preventDefault()
                    signIn()
                    }}>Sign In</a>
                </li>
                </>
              )
            }
            {session?.user && (
              <div className="profholder">
              <li>
              <Image src={session?.user.image} width={500} height={500} alt={"avatar"} className="profpic" onClick={(e) => {router.push('/profile')}}/>
              <a><strong>{session?.user.name}</strong></a>
              </li>
              <li>
                <a onClick={(e) => {
                  e.preventDefault()
                  signOut()
                }}>Sign out</a>
              </li>
              </div>
            )}
          </ul>
        </div>
      </nav>
    );
}

export default Navbar;