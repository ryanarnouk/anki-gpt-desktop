import { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import "../App.css";
import { emit, listen } from '@tauri-apps/api/event';

interface Question {
    question: string,
    answer: string
}

const unselectedQuestion: Question = {
    question: "Could not get question from Anki (make sure you have the app open on a card)",
    answer: "n/a"
};

function AnswerPage() {
    const [card, setCard] = useState<Question>(unselectedQuestion);
    const [text, setText] = useState("");
    const [grade, setGrade] = useState("No grade from the model yet!");

    const listener = async () => {
        try {
            await listen("new_card", (event) => {
                const parsedObject: Question = JSON.parse(event.payload as string);
                setCard(parsedObject);
                setGrade("No grade from the model yet!");
            });
        } catch (error) {
            console.log("error listening for Tauri event", error);
        }
    }

    useEffect(() => {
        emit("resume");
        
        listener();

        return () => {
            emit("pause");
        }
    }, []);

    const handleTextChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        setText(event.target.value);
    }

    const onGrade = async () => {
        setGrade("Loading...");
        const grade: string = await invoke("score_answer", {question: card.question, userAnswer: text, aiAnswer: card.answer});
        setGrade(grade);
    }

    return (
        <div className="container">
            <h1>Anki GPT Integration</h1>

            <p>{card.question}</p>
            <textarea 
                value={text}
                onChange={handleTextChange}
                rows={10}
                cols={50}
            />
            <p>{grade}</p>
            <button type="submit" onClick={onGrade}>Answer</button>
        </div>
    );
}

export default AnswerPage;
