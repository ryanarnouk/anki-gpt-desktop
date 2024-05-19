import { useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import "../App.css";

function AnswerPage() {
    const [key, setKey] = useState("");

    const handleTextChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        setKey(event.target.value);
    }

    const onUpdateKey = async () => {
        await invoke("save_api_key", { key: key });
        setKey("");
    }

    return (
        <div className="container">
            <h1>Anki GPT Integration</h1>

            <textarea 
                value={key}
                onChange={handleTextChange}
                rows={10}
                cols={50}
            />
            <br/>
            <button type="submit" onClick={onUpdateKey}>Update OPENAI key</button>
        </div>
    );
}

export default AnswerPage;
