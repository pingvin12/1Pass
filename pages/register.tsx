import Head from "next/head";
import Image from "next/image";
import SimpleReactValidator from "simple-react-validator";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import "./api/auth/auth";
import { registerAsync } from "./api/auth/auth";
export default function Register() {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const router = useRouter();
  const validator = new SimpleReactValidator();
  const [register, registerClicked] = useState(false);

  useEffect(() => {
    const completeForm = async () => {
      if (typeof window !== "undefined") {
        if (validator.allValid()) {
          try {
            const data = registerAsync(email, username, password);
            data.then((val) => {
              router.push("misc/completedRegistration");
              return val;
            });
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
  }, [register]);

  const forceUpdate = () => {
    setUsername(username);
    setEmail(email);
    setPassword(password);
    console.log(email + username + password);
  };

  return (
    <div className="register-container-container">
      <div className="register-container">
        <div className="register-header-container">
          <span className="register-header">Register</span>
        </div>
        <div className="register-input-container">
          <input
            type="text"
            name="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="youremail@example.com"
          />
          {validator.message("Email", email, "required|email")}
        </div>
        <div className="register-username-container">
          <input
            type="text"
            name="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            placeholder="Username"
          />
          {validator.message("Username", username, "required")}
        </div>
        <div className="register-password-container">
          <input
            type="password"
            name="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="*********"
          />
          {validator.message("Password", password, "required|min:8")}
        </div>
        <div className="register-button-container">
          <button onClick={(e) => registerClicked(true)}>Register</button>
        </div>
      </div>
    </div>
  );
}
