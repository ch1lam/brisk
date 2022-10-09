import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";
import { register } from "@tauri-apps/api/globalShortcut";
import { invoke } from "@tauri-apps/api/tauri"

// const invoke = window.__TAURI__.invoke

register('Alt+P', () => {
  console.log('Alt+Enter pressed!')
  invoke('evoke_main_command')
})

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
