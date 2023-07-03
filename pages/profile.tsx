import React, { useEffect, useState } from "react";
import Layout from "../Components/BaseComponents/layout";
import { me } from "./api/auth/auth";
export default function Register() {
    const [userData, setUserData] = useState({
        name: "",
        email: "",
        jwtToken: {
          token: "",
          validity: 0,
        },
      });
      
    useEffect(() => {
        const fetchData = async () => {
            if (typeof window !== "undefined") {
              const authtoken: string | null = localStorage.getItem("token");
              console.log(authtoken);
              const token = await me(authtoken as string);
              if (token !== undefined && authtoken) {
                const user = JSON.parse(token);
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
          }
          };
          if (typeof window !== "undefined") {
            fetchData();
          }
    }, []);
  
    return (
    <Layout>
      <h1>Profile</h1>
        <p>Name: {userData.name}</p>
        <p>Email: {userData.email}</p>
        
    </Layout>
  );
}
