import Head from "next/head";
import Image from "next/image";
import SimpleReactValidator from "simple-react-validator";
import { useRouter } from "next/router";
// When using the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoker = window.__TAURI__.invoke;

export default function Register() {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const router = useRouter();
  const validator = new SimpleReactValidator();
  function completeForm() {
    console.log(validator.allValid());
    if (validator.allValid()) {
      invoker("command_register_user", {
        username: username,
        password: password,
        email: email,
      }).then(() => {
        router.push("misc/completedRegistration");
      });
    } else {
      forceUpdate();
      validator.showMessages();
    }
  }

  const forceUpdate = () => {
    setUsername(username);
    setEmail(email);
    setPassword(password);
    console.log(email + username + password);
  };

  return (
    <div className="rounded-xl bg-white bg-opacity-50 px-16 py-10 shadow-lg backdrop-blur-md max-sm:px-8">
      <div className="text-white">
        <div className="mb-8 flex flex-col items-center">
          <h1 className="mb-2 text-2xl">1Pass</h1>
          <span className="text-gray-300">Register</span>
        </div>
        <div className="mb-4 text-lg">
          <input
            type="text"
            name="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="youremail@email.com"
          />
          {validator.message("Email", email, "required|email")}
        </div>
        <div className="mb-4 text-lg">
          <input
            type="text"
            name="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            placeholder="Username"
          />
          {validator.message("Username", username, "required")}
        </div>
        <div className="mb-4 text-lg">
          <input
            type="password"
            name="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="*********"
          />
          {validator.message("Password", password, "required|min:8")}
        </div>
        <div className="mt-8 flex justify-center text-lg text-black">
          <button onClick={() => completeForm()}>Register</button>
        </div>
      </div>
    </div>
  );
}
