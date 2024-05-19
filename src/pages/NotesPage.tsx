import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "../App.css";
import FileSelector from "../components/FileSelector";

function NotesPage() {
  const [name, setName] = useState("");
  const [text, setText] = useState("");

  const generate = async (e: React.ChangeEvent<HTMLFormElement>) => {
    e.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("generate_question_answers", { result: text, deckName: name });
    console.log("the questions have been generated");
    setName("");
    setText("");
  }

  const handleTextChange = (newState: string) => {
    setText(newState);
  }

  return (
    <div className="container">
      <h1>Anki GPT Integration</h1>

      <p>Upload Notes</p>
      <FileSelector onTextChange={handleTextChange} text={text}/>
      <form
        className="row"
        onSubmit={generate}
      >
        <input
          id="generate-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Deck Name"
          value={name}
        />
        <button type="submit">Generate</button>
      </form>
    </div>
  );
}

export default NotesPage;
