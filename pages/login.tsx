import Head from "next/head";
import Image from "next/image";
import { useContext, useEffect, useState } from "react";
import SimpleReactValidator from "simple-react-validator";
import { invoke } from "@tauri-apps/api/tauri";
import { authObject } from "./api/models/auth.object";
import dynamic from "next/dynamic";
import { loginAsync } from "./api/auth/auth";
import router from "next/router";
import { useSWRConfig } from "swr";

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
    if (typeof window !== "undefined") {
      if (validator.allValid() && login) {
        try {
          const data = loginAsync(email, password);
          console.log(data);
          router.push("home");
        } catch (error) {
          console.error("Login page error", error);
        }
      } else {
        validator.showMessages();
        forceUpdate();
      }
    }
  }, [login]);

  return (
    <div className="login-container-container">
      <div className="login-container">
        <div className="login-span-header-container">
          <span className="login-span-header">Login</span>
        </div>
        <div className="login-email-error-container">
          <input
            type="text"
            name="Email"
            onChange={(e) => setEmail(e.target.value)}
            placeholder="youremail@email.com"
          />
          {validator.message("email", email, "required")}
        </div>
        <div className="login-password-error-container">
          <input
            type="password"
            name="Password"
            onChange={(e) => setPassword(e.target.value)}
            placeholder="*********"
          />
          {validator.message("password", password, "required")}
        </div>
        <div className="login-btn-container">
          <button onClick={(e) => loginClicked(true)}>Login</button>
        </div>
      </div>
    </div>
  );
}
