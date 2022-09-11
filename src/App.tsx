import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { appWindow } from "@tauri-apps/api/window";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <input
      id="greet-input"
      onChange={(e) => setName(e.currentTarget.value)}
      placeholder="Enter a name..."
    />
  );
}

export default App;
