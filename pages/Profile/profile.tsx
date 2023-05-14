import React from 'react'
import Layout from '../Components/layout'
import { useSession } from 'next-auth/react'
import Image from "next/image"
export default function Profile() {
    const {data: session, status } = useSession()
        return (
            <>
            <Layout>
            {session?.user ? <>
                <span style={{backgroundImage: `url('${session?.user?.image}')` }}></span><br/>
            <strong>Name: {session?.user?.name}</strong><br/>
            <strong>Email: {session?.user?.email}</strong>
            </>
           
            :
            <>
           <a>You are not logged in</a>
</>
            
            }
            </Layout>
            </>
        )
    
    
}