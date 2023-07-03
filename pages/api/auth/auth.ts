import { createContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import useSWR, { mutate } from "swr";
import { authObject } from "../models/auth.object";
import { invokeFetcher } from "../invoke/useInvoke";

export const loginAsync = async (
  email: string,
  password: string
): Promise<string | undefined> => {
  try {
    const key = ["auth", { email, password }];
    const data = await invokeFetcher(key);
    localStorage.setItem("token", `${data}`);
    return data as string;
  } catch (err) {
    console.error("Login failed", err);
    return undefined;
  }
};

export const me = async (token: string): Promise<string | undefined> => {
  try {
    const data: unknown = await invokeFetcher(["me", { token }]);
    return data as string;
  } catch (err) {
    console.error("Get user failed", err);
    return undefined;
  }
};

export const logoutAsync = async () => {
  try {
    localStorage.removeItem("token");
  } catch (err) {
    console.error("logout error", err);
  }
};

export const registerAsync = async (
  email: string,
  username: string,
  password: string
) => {
  try {
    const key = ["register", { email, username, password }];
    const data = await invokeFetcher(key);
    console.log(data);
    return data;
  } catch (error) {
    console.error("Register failed", error);
    throw error;
  }
};
