import React, { useEffect, useState } from "react";
import CardLayout from "../Components/card_layout";
import Card from "../Components/BaseComponents/card";
import { get_secrets } from "./api/secret/secret";
import { secretObject } from "./api/models/secret.object";
import FloatingAddButton from "../Components/BaseComponents/add_btn";
import EmptySecretsOrNotLoggedInModal from "../Components/Modals/EmptySecretsOrNotLoggedInModal";
import router from "next/router";
export default function Register() {

  const [secrets, setSecrets] = useState<secretObject[] | undefined>([]);

  const fetchSecrets = async () => {
    let token = localStorage.getItem("token");
    if (typeof window !== "undefined" && token) {
      const data = await get_secrets(token);
      if(data !== undefined) {
        console.log(data);
        setSecrets(data);  
      }
    }}

  useEffect(() => {
      fetchSecrets();
    }, []);

  return (
    <div className="rounded-xl bg-white bg-opacity-50 px-16 py-10 shadow-lg backdrop-blur-md max-sm:px-8">
      <CardLayout>
        {
          secrets ?
          secrets.map((secret: secretObject) => {
            return <Card title={secret.title} description={secret.content} id={secret.id} onRemove={fetchSecrets} />
          }) : ""
        }
      </CardLayout>

      {
        localStorage.getItem("token") ? <FloatingAddButton onAdd={fetchSecrets} /> : 
        <>
        <EmptySecretsOrNotLoggedInModal onClose={function (): void {
              router.push("/login");
            } } />
        </>
      }
      
    </div>
  );
}
