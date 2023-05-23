import React from "react";
import ThemeToggleSwitch from "../Components/Settings/themeswitch";
export default function Settings() {
    return (
     <div className="rounded-xl bg-white bg-opacity-50 px-16 py-10 shadow-lg backdrop-blur-md max-sm:px-8">
      <div className="text-white">
        <ThemeToggleSwitch/>
      </div>
    </div>
    )
  }