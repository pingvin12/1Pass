import Head from "next/head";
import Image from "next/image";
import { useEffect, useState } from "react";
import SimpleReactValidator from "simple-react-validator";
import { invoke } from "@tauri-apps/api/tauri";

interface LoginProps {
  validator: SimpleReactValidator;
}

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const validator = new SimpleReactValidator();
  const [login, loginClicked] = useState(false);
  const forceUpdate = () => {
    setEmail(email);
    setPassword(password);
  };

  useEffect(() => {
    const completeForm = async () => {
      if (typeof window !== "undefined") {
        if (validator.allValid()) {
          try {
            const token: string = await invoke("command_login_user", {
              email,
              password,
            });
            console.log(token);
          } catch (error) {
            console.error(error);
          }
        } else {
          validator.showMessages();
          forceUpdate();
        }
      }
    };

    completeForm();
  }, [login]);

  return (
    <div className="rounded-xl bg-white bg-opacity-50 px-16 py-10 shadow-lg backdrop-blur-md max-sm:px-8">
      <div className="text-white">
        <div className="mb-8 flex flex-col items-center">
          <h1 className="mb-2 text-2xl">1Pass</h1>
          <span className="text-gray-300">Login</span>
        </div>
        <div className="mb-4 text-lg">
          <input
            type="text"
            name="Email"
            onChange={(e) => setEmail(e.target.value)}
            placeholder="youremail@email.com"
          />
          {validator.message("email", email, "required")}
        </div>
        <div className="mb-4 text-lg">
          <input
            type="password"
            name="Password"
            onChange={(e) => setPassword(e.target.value)}
            placeholder="*********"
          />
          {validator.message("password", password, "required")}
        </div>
        <div className="mt-8 flex justify-center text-lg text-black">
          <button onClick={(e) => loginClicked(true)}>Login</button>
        </div>
      </div>
    </div>
  );
}
