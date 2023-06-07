import React, { createContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { authObject } from "../models/auth.object";

const AuthContext = createContext({
  user: null,
});

export function AuthProvider({ children }) {
  const [loginState, setLoginState] = useState({
    loggedIn: false,
    name: "",
    email: "",
    jwtToken: {
      token: "",
      validity: 0,
    },
  });
  const loginAsync = async (email: string, password: string) => {
    try {
      const token: authObject = await invoke("auth", {
        email,
        password,
      });

      setLoginState({
        loggedIn: true,
        name: token.name,
        email: token.email,
        jwtToken: token.jwtToken,
      });
    } catch (err) {
      console.error("Login failed", err);
    }
  };

  const logoutAsync = async () => {
    try {
      if (loginState.loggedIn === true) {
        const response: boolean = await invoke("logout", {
          token: loginState.jwtToken.token,
        });

        return response;
      }
    } catch (err) {
      console.error("logout error", err);
    }
  };

  const registerAsync = async (
    email: string,
    username: string,
    password: string
  ) => {
    try {
      const response: boolean = await invoke("register", {
        email,
        username,
        password,
      });

      return response;
    } catch (err) {
      console.error("logout error", err);
    }
  };

  const checkUserLoggedIn = async () => {
    try {
      const response: boolean = await invoke("check_auth", {
        token: loginState.jwtToken.token,
      });

      return response;
    } catch (err) {
      console.error("check auth error", err);
    }
  };

  useEffect(() => {
    // Check for user authentication status on initial load
    checkUserLoggedIn();
  }, []);
}
