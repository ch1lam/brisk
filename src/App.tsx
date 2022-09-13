import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import styled from "@emotion/styled";

function App() {
  return (
    <Container>
      <SearchBox id="greet-input" placeholder="Brisk..." />
    </Container>
  );
}

const Container = styled.div`
  margin: 0;
  padding: 0;
  display: flex;
  height: 100%;
  width: 100%;
  justify-content: center;
  align-items: center;
`;
const SearchBox = styled.input`
  box-sizing: border-box;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  border-radius: 10px;
  outline: none;
  border: 1px dashed black;
  padding: 0.6em 1.2em;
  font-size: 1.2em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
`;

export default App;
